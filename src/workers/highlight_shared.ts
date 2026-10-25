export const order_types = [
    // ─────────────────────────────
    // Base
    // ─────────────────────────────
    "unknown",
    "expression",
    "identifier",
    "property",
    "operator",
    "punctuation",

    // ─────────────────────────────
    // Literals
    // ─────────────────────────────

    "string",
    "char",
    "number",
    "bool",
    "null",
    "regex",
    "template",

    // ─────────────────────────────
    // Declarations / types
    // ─────────────────────────────

    "type",
    "type_builtin",
    "generic",
    "param",
    "argument",

    "const",

    "fn_decl",
    "fn_call",
    "fn_b",

    "a_fn_decl",
    "a_fn_call",
    "a_fn_b",

    "spec_fn",

    "entity_stat",

    // ─────────────────────────────
    // Modifiers / keywords
    // ─────────────────────────────

    "modifier",
    "visibility",
    "storage",

    "other_kw_decl",
    "other_kw_stat",

    // ─────────────────────────────
    // Control flow
    // ─────────────────────────────

    "if_stat",

    "while_stat",
    "for_stat",
    "loop_stat",

    "match_in",
    "match_stat",

    "util_stat",
    "util_decl",

    // ─────────────────────────────
    // Exceptions
    // ─────────────────────────────

    "exception",
    "exception_decl",

    // ─────────────────────────────
    // Modules
    // ─────────────────────────────

    "scope",
    "import",
    "export",

    // ─────────────────────────────
    // Special
    // ─────────────────────────────

    "decorator",
    "annotation",
    "attribute",
    "macro",

    // ─────────────────────────────
    // Comments / documentation
    // ─────────────────────────────

    "comment",
    "doc_comment",

    // ─────────────────────────────
    // Misc
    // ─────────────────────────────

    "label",
    "escape",
    "error",
    "other",
] as const


/**
 * @deprecated
 * @param typ
 * @param arr
 */
export function get_type_from_arr(typ: string, arr: Record<string, string>): typeof order_types[number] {
    let type_ = arr[typ] as string | undefined;
    console.log("\t\t\t got", typ, type_)
    if (typeof type_ == "string" && order_types.includes(type_ as typeof order_types[number])) {
        return type_ as typeof order_types[number]
    }
    return "unknown"
}

/**
 * @deprecated
 * @param typ
 */
export function get_order(typ: typeof order_types[number]): number {
    let index = order_types.indexOf(typ)
    return index ?? 0
}


export interface Token {
    typ: typeof order_types[number],
    text: string,
    start: number,
    end: number
}

export interface ExtToken extends Token {
    color: string
}
