use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Кикс" => "Err",
        "ВоРед" => "Ok",
        "НизаОдКарактери" => "String",
        "Речник" => "HashMap",
        "Основна" => "Default",
        "Грешка" => "Error",
        "Опција" => "Option",
        "Некои" => "Some",
        "Ништо" => "None",
        "Резултат" => "Result",
        "Суштина" => "Self",
        "ИспишиРед" => "println",
        "Запри" => "break",
        "асинхронизирано" => "async",
        "почекај" => "await",
        "boucle" => "loop",
        "премести" => "move",
        "гајба" => "crate",
        "недостапен_код" => "unreachable_code",
        "како" => "as",
        "постојано" => "const",
        "својство" => "trait",
        "опасно" => "unsafe",
        "во" => "in",
        "од" => "from",
        "дин" => "dyn",
        "одмотај" => "unwrap",
        "основно" => "default",
        "како_реф" => "as_ref",
        "ио" => "io",
        "надорешно" => "extern",
        "неточно" => "false",
        "фн" => "fn",
        "одлично" => "super",
        "внеси" => "insert",
        "земи" => "get",
        "дозволи" => "allow",
        "паника" | "calisse" | "oups" => "panic",
        "модул" => "mod",
        "променливо" => "mut",
        "ново" => "new",
        "каде" => "where",
        "за" => "for",
        "земи_или_внеси_со" => "get_or_insert_with",
        "главно" => "main",
        "јавно" => "pub",
        "тоа" => None?,
        "врати" => "return",
        "полнење" => "impl", // ?
        "реф" => "ref",
        "совпадне" => "match",
        "ако" => "if",
        "тогаш" => "else",
        "свое" => "self",
        "дозволи" => "let",
        "статично" => "static",
        "структура" => "struct",
        "очекувај" => "expect",
        "додека" => "while",
        "користи" => "use",
        "претвори" => "into",
        "вистина" => "true",
        "набројување" => "enum",
        "Група" => "Group",
        "Identifiant" => "Ident",
        "ИзворНаТокен" => "TokenStream",
        "ДрвоНаТокен" => "TokenTree",
        "во_стр" => "to_string",
        "како_низа_карактери" => "as_str",
        "portée" => "span",
        "Листа" => "Vec",
        "извор" => "stream",
        "бутни" => "push",
        "продолжи" => "extend",
        "Ограничувач" => "delimiter",
        "Точка" => "Punct",
        "Буквално" => "Literal",
        "макро_процес" => "proc_macro",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rouille(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
