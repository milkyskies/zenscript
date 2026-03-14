type Todo = {
    id: string,
    text: string,
    done: bool,
}

type Filter =
    | All
    | Active
    | Completed

type Validation =
    | Valid(text: string)
    | TooShort
    | TooLong
    | Empty
