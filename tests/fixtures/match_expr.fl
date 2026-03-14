type Route =
    | Home
    | Profile(id: string)
    | NotFound

const route = Home

const _page = match route {
    Home -> "home",
    Profile(id) -> id,
    NotFound -> "404",
}
