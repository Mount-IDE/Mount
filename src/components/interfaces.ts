

interface RecentProject{
    name: string,
    path: string
}


interface IProject {
    name: string,
    path: string,
    meta: {
        authors: string[],
        description: string,
        license: string,
        tags: string[],
        group: string
    },
    packages: string[]
}