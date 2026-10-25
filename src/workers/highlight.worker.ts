import {Language, Parser, Query, Tree} from "web-tree-sitter";

import {order_types, type Token} from "./highlight_shared";

type Lang = { parser: Parser; query: Query };

const ready = Parser.init({
    "locateFile": (path: string) => "/" + path
});

const langs = new Map<string, Lang>();
const trees = new Map<string, Tree>();       // ключ: fileId
const texts = new Map<string, string>();     // предыдущий текст файла

type InMsg =
    | { type: "register"; key: string; wasmUrl: Uint8Array; querySource: string }
    | { type: "highlight"; reqId: number; fileId: string; key: string; text: string; nodes: Record<string, string> }
    | { type: "close"; fileId: string };


self.onmessage = async (e: MessageEvent<InMsg>) => {
    const msg = e.data;
    await ready;

    if (msg.type === "register") {
        if (langs.has(msg.key)) return;
        const lang = await Language.load(msg.wasmUrl);
        const parser = new Parser();
        parser.setLanguage(lang);
        langs.set(msg.key, {parser, query: new Query(lang, msg.querySource)});
        return;
    }

    if (msg.type === "close") {
        trees.get(msg.fileId)?.delete();
        trees.delete(msg.fileId);
        texts.delete(msg.fileId);
        return;
    }

    // highlight
    const lang = langs.get(msg.key);
    if (!lang) {
        self.postMessage({type: "result", reqId: msg.reqId, fileId: msg.fileId, lines: null});
        return;
    }

    const old = trees.get(msg.fileId);
    const oldText = texts.get(msg.fileId);
    if (old && oldText !== undefined) {
        //@ts-ignore
        old.edit(diffToEdit(oldText, msg.text));
    }

    const tree = lang.parser.parse(msg.text, old ?? undefined);
    old?.delete();
    if (!tree) return;
    trees.set(msg.fileId, tree);
    texts.set(msg.fileId, msg.text);

    const captures = lang.query.captures(tree.rootNode);
    const tokens = buildTokens(msg.text, captures, msg.nodes);
    const lines = splitByLines(msg.text, tokens);

    self.postMessage({type: "result", reqId: msg.reqId, fileId: msg.fileId, lines});
};


function pointAt(text: string, index: number) {
    let row = 0, last = -1;
    for (let i = text.indexOf("\n"); i !== -1 && i < index; i = text.indexOf("\n", i + 1)) {
        row++;
        last = i;
    }
    return {row, column: index - (last + 1)};
}

function diffToEdit(a: string, b: string) {
    let start = 0;
    const min = Math.min(a.length, b.length);
    while (start < min && a.charCodeAt(start) === b.charCodeAt(start)) start++;

    let endA = a.length, endB = b.length;
    while (endA > start && endB > start && a.charCodeAt(endA - 1) === b.charCodeAt(endB - 1)) {
        endA--;
        endB--;
    }
    return {
        startIndex: start,
        oldEndIndex: endA,
        newEndIndex: endB,
        startPosition: pointAt(a, start),
        oldEndPosition: pointAt(a, endA),
        newEndPosition: pointAt(b, endB),
    };
}


function buildTokens(text: string, captures: any[], dict: Record<string, string>): Token[] {
    const map = new Map<string, Token>();
    for (const c of captures) {
        const start = c.node.startIndex, end = c.node.endIndex;
        const key = `${start}:${end}`;
        const raw = dict[c.name];
        const typ = (order_types as readonly string[]).includes(raw) ? raw : "unknown";
        const got = map.get(key);
        if (!got || order_types.indexOf(typ as any) > order_types.indexOf(got.typ)) {
            map.set(key, {start, end, typ: typ as any, text: ""});
        }
    }

    const sorted = [...map.values()].sort((a, b) => a.start - b.start);
    const result: Token[] = [];
    let pos = 0;
    for (const t of sorted) {
        if (pos < t.start) result.push({start: pos, end: t.start, typ: "unknown", text: ""});
        result.push(t);
        pos = Math.max(pos, t.end);
    }
    if (pos < text.length) result.push({start: pos, end: text.length, typ: "unknown", text: ""});
    return result;
}

function splitByLines(text: string, tokens: Token[]): Map<number, Token[]> {
    const lineStarts = [0];
    for (let i = text.indexOf("\n"); i !== -1; i = text.indexOf("\n", i + 1)) lineStarts.push(i + 1);

    const lineOf = (off: number) => {
        let lo = 0, hi = lineStarts.length - 1;
        while (lo < hi) {
            const mid = (lo + hi + 1) >> 1;
            if (lineStarts[mid] <= off) lo = mid; else hi = mid - 1;
        }
        return lo;
    };

    const byLine = new Map<number, Token[]>();
    const push = (l: number, t: Token) => {
        let arr = byLine.get(l);
        if (!arr) byLine.set(l, arr = []);
        arr.push(t);
    };

    for (const t of tokens) {
        const l1 = lineOf(t.start);
        const l2 = lineOf(Math.max(t.start, t.end - 1));
        for (let l = l1; l <= l2; l++) {
            const ls = lineStarts[l];
            const le = lineStarts[l + 1] !== undefined ? lineStarts[l + 1] - 1 : text.length;
            const s = Math.max(t.start, ls), e = Math.min(t.end, le);
            if (e > s) push(l, {...t, start: s, end: e, text: text.slice(s, e)});
        }
    }
    return byLine;
}