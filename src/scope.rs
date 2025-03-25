pub struct Scope {
    Variables: HashMap<&'static str, Token>,
    Functions: HashMap<&'static str, ULispFunc>,
}
