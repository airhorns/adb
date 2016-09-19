use super::ast::*;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Statement {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use super::super::ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_21_3d_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2e_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3c_22(&'input str),
        Term_22_3c_3d_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22_3e_22(&'input str),
        Term_22_3e_3d_22(&'input str),
        Term_22FALSE_22(&'input str),
        Term_22FROM_22(&'input str),
        Term_22INSERT_20INTO_22(&'input str),
        Term_22NULL_22(&'input str),
        Term_22SELECT_22(&'input str),
        Term_22TRUE_22(&'input str),
        Term_22WHERE_22(&'input str),
        Term_22r_5b0_2d9_5d_2b_22(&'input str),
        Term_22r_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2a_22(&'input str),
        Term_22r_5c_5c_2e_5b0_2d9_5d_2b_22(&'input str),
        Termr_23_22_5c_22_5b_5e_5c_22_5d_2a_5c_22_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ___5d_5ba_2dzA_2dZ0_2d9_40_3a___5d_2a_22_23(&'input str),
        Nt_22_3b_22_3f(::std::option::Option<&'input str>),
        Nt_28_22_2e_22_20_3cIdentifier_3e_29(Identifier),
        Nt_28_22_2e_22_20_3cIdentifier_3e_29_2a(::std::vec::Vec<Identifier>),
        Nt_28_22_2e_22_20_3cIdentifier_3e_29_2b(::std::vec::Vec<Identifier>),
        Nt_28_3cRelation_3e_20_22_2c_22_29(Relation),
        Nt_28_3cRelation_3e_20_22_2c_22_29_2a(::std::vec::Vec<Relation>),
        Nt_28_3cRelation_3e_20_22_2c_22_29_2b(::std::vec::Vec<Relation>),
        Nt_28_3cSelectItem_3e_20_22_2c_22_29(SelectItem),
        Nt_28_3cSelectItem_3e_20_22_2c_22_29_2a(::std::vec::Vec<SelectItem>),
        Nt_28_3cSelectItem_3e_20_22_2c_22_29_2b(::std::vec::Vec<SelectItem>),
        NtBoolean(&'input str),
        NtBooleanExpression(Expression),
        NtComma_3cRelation_3e(Vec<Relation>),
        NtComma_3cSelectItem_3e(Vec<SelectItem>),
        NtComparisonOperator(ComparisonOperator),
        NtExpression(Expression),
        NtIdentifier(Identifier),
        NtInsert(InsertStatement),
        NtNumber(&'input str),
        NtQualifiedName(QualifiedName),
        NtQuery(SelectStatement),
        NtQueryFrom(Vec<Relation>),
        NtQueryFrom_3f(::std::option::Option<Vec<Relation>>),
        NtQueryWhere(Box<Expression>),
        NtQueryWhere_3f(::std::option::Option<Box<Expression>>),
        NtRelation(Relation),
        NtSelectItem(SelectItem),
        NtStatement(Box<Statement>),
        NtValueExpression(Expression),
        Nt____Statement(Box<Statement>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        5, // on "INSERT INTO", goto 4
        0, // on "NULL", error
        6, // on "SELECT", goto 5
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 1
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        7, // on ";", goto 6
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 2
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        8, // on ";", goto 7
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 3
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 4
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        11, // on r#"\"[^\"]*\""#, goto 10
        12, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 11
        // State 5
        0, // on "!=", error
        21, // on "(", goto 20
        0, // on ")", error
        22, // on "*", goto 21
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        23, // on "FALSE", goto 22
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        24, // on "NULL", goto 23
        0, // on "SELECT", error
        25, // on "TRUE", goto 24
        0, // on "WHERE", error
        26, // on "r[0-9]+", goto 25
        27, // on "r[0-9]+\\.[0-9]*", goto 26
        28, // on "r\\.[0-9]+", goto 27
        29, // on r#"\"[^\"]*\""#, goto 28
        30, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 29
        // State 6
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 7
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 8
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        32, // on ".", goto 31
        -51, // on ";", reduce `Relation = Identifier => ActionFn(72);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 9
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -35, // on ";", reduce `Insert = "INSERT INTO", Relation => ActionFn(6);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 10
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        -33, // on ".", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ";", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 11
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        -34, // on ".", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ";", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 12
        -63, // on "!=", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "(", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on ")", error
        -63, // on "*", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on ",", error
        0, // on ".", error
        -63, // on ";", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "<", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "<=", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "=", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on ">", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on ">=", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "FALSE", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "FROM", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on "INSERT INTO", error
        -63, // on "NULL", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on "SELECT", error
        -63, // on "TRUE", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "WHERE", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "r[0-9]+", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "r[0-9]+\\.[0-9]*", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "r\\.[0-9]+", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on r#"\"[^\"]*\""#, reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ValueExpression = Boolean => ActionFn(19);`
        // State 13
        0, // on "!=", error
        -32, // on "(", reduce `Expression = BooleanExpression => ActionFn(15);`
        0, // on ")", error
        -32, // on "*", reduce `Expression = BooleanExpression => ActionFn(15);`
        0, // on ",", error
        0, // on ".", error
        -32, // on ";", reduce `Expression = BooleanExpression => ActionFn(15);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -32, // on "FALSE", reduce `Expression = BooleanExpression => ActionFn(15);`
        -32, // on "FROM", reduce `Expression = BooleanExpression => ActionFn(15);`
        0, // on "INSERT INTO", error
        -32, // on "NULL", reduce `Expression = BooleanExpression => ActionFn(15);`
        0, // on "SELECT", error
        -32, // on "TRUE", reduce `Expression = BooleanExpression => ActionFn(15);`
        -32, // on "WHERE", reduce `Expression = BooleanExpression => ActionFn(15);`
        -32, // on "r[0-9]+", reduce `Expression = BooleanExpression => ActionFn(15);`
        -32, // on "r[0-9]+\\.[0-9]*", reduce `Expression = BooleanExpression => ActionFn(15);`
        -32, // on "r\\.[0-9]+", reduce `Expression = BooleanExpression => ActionFn(15);`
        -32, // on r#"\"[^\"]*\""#, reduce `Expression = BooleanExpression => ActionFn(15);`
        -32, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Expression = BooleanExpression => ActionFn(15);`
        // State 14
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -44, // on ";", reduce `Query = "SELECT", Comma<SelectItem> => ActionFn(83);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        35, // on "FROM", goto 34
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        36, // on "WHERE", goto 35
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 15
        0, // on "!=", error
        -53, // on "(", reduce `SelectItem = Expression => ActionFn(7);`
        0, // on ")", error
        -53, // on "*", reduce `SelectItem = Expression => ActionFn(7);`
        0, // on ",", error
        0, // on ".", error
        -53, // on ";", reduce `SelectItem = Expression => ActionFn(7);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -53, // on "FALSE", reduce `SelectItem = Expression => ActionFn(7);`
        -53, // on "FROM", reduce `SelectItem = Expression => ActionFn(7);`
        0, // on "INSERT INTO", error
        -53, // on "NULL", reduce `SelectItem = Expression => ActionFn(7);`
        0, // on "SELECT", error
        -53, // on "TRUE", reduce `SelectItem = Expression => ActionFn(7);`
        -53, // on "WHERE", reduce `SelectItem = Expression => ActionFn(7);`
        -53, // on "r[0-9]+", reduce `SelectItem = Expression => ActionFn(7);`
        -53, // on "r[0-9]+\\.[0-9]*", reduce `SelectItem = Expression => ActionFn(7);`
        -53, // on "r\\.[0-9]+", reduce `SelectItem = Expression => ActionFn(7);`
        -53, // on r#"\"[^\"]*\""#, reduce `SelectItem = Expression => ActionFn(7);`
        -53, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `SelectItem = Expression => ActionFn(7);`
        // State 16
        -64, // on "!=", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "(", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on ")", error
        -64, // on "*", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on ",", error
        38, // on ".", goto 37
        -64, // on ";", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "<", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "<=", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "=", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on ">", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on ">=", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "FALSE", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "FROM", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on "INSERT INTO", error
        -64, // on "NULL", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on "SELECT", error
        -64, // on "TRUE", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "WHERE", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "r[0-9]+", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "r[0-9]+\\.[0-9]*", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "r\\.[0-9]+", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on r#"\"[^\"]*\""#, reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ValueExpression = Identifier => ActionFn(76);`
        // State 17
        -62, // on "!=", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "(", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on ")", error
        -62, // on "*", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on ",", error
        0, // on ".", error
        -62, // on ";", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "<", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "<=", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "=", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on ">", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on ">=", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "FALSE", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "FROM", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on "INSERT INTO", error
        -62, // on "NULL", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on "SELECT", error
        -62, // on "TRUE", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "WHERE", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "r[0-9]+", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "r[0-9]+\\.[0-9]*", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "r\\.[0-9]+", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on r#"\"[^\"]*\""#, reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ValueExpression = Number => ActionFn(18);`
        // State 18
        0, // on "!=", error
        47, // on "(", goto 46
        0, // on ")", error
        48, // on "*", goto 47
        0, // on ",", error
        0, // on ".", error
        -23, // on ";", reduce `Comma<SelectItem> = SelectItem => ActionFn(70);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        49, // on "FALSE", goto 48
        -23, // on "FROM", reduce `Comma<SelectItem> = SelectItem => ActionFn(70);`
        0, // on "INSERT INTO", error
        50, // on "NULL", goto 49
        0, // on "SELECT", error
        51, // on "TRUE", goto 50
        -23, // on "WHERE", reduce `Comma<SelectItem> = SelectItem => ActionFn(70);`
        52, // on "r[0-9]+", goto 51
        53, // on "r[0-9]+\\.[0-9]*", goto 52
        54, // on "r\\.[0-9]+", goto 53
        55, // on r#"\"[^\"]*\""#, goto 54
        56, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 55
        // State 19
        58, // on "!=", goto 57
        -31, // on "(", reduce `Expression = ValueExpression => ActionFn(14);`
        0, // on ")", error
        -31, // on "*", reduce `Expression = ValueExpression => ActionFn(14);`
        0, // on ",", error
        0, // on ".", error
        -31, // on ";", reduce `Expression = ValueExpression => ActionFn(14);`
        59, // on "<", goto 58
        60, // on "<=", goto 59
        61, // on "=", goto 60
        62, // on ">", goto 61
        63, // on ">=", goto 62
        -31, // on "FALSE", reduce `Expression = ValueExpression => ActionFn(14);`
        -31, // on "FROM", reduce `Expression = ValueExpression => ActionFn(14);`
        0, // on "INSERT INTO", error
        -31, // on "NULL", reduce `Expression = ValueExpression => ActionFn(14);`
        0, // on "SELECT", error
        -31, // on "TRUE", reduce `Expression = ValueExpression => ActionFn(14);`
        -31, // on "WHERE", reduce `Expression = ValueExpression => ActionFn(14);`
        -31, // on "r[0-9]+", reduce `Expression = ValueExpression => ActionFn(14);`
        -31, // on "r[0-9]+\\.[0-9]*", reduce `Expression = ValueExpression => ActionFn(14);`
        -31, // on "r\\.[0-9]+", reduce `Expression = ValueExpression => ActionFn(14);`
        -31, // on r#"\"[^\"]*\""#, reduce `Expression = ValueExpression => ActionFn(14);`
        -31, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Expression = ValueExpression => ActionFn(14);`
        // State 20
        0, // on "!=", error
        70, // on "(", goto 69
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        71, // on "FALSE", goto 70
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        72, // on "NULL", goto 71
        0, // on "SELECT", error
        73, // on "TRUE", goto 72
        0, // on "WHERE", error
        74, // on "r[0-9]+", goto 73
        75, // on "r[0-9]+\\.[0-9]*", goto 74
        76, // on "r\\.[0-9]+", goto 75
        77, // on r#"\"[^\"]*\""#, goto 76
        78, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 77
        // State 21
        0, // on "!=", error
        -56, // on "(", reduce `SelectItem = "*" => ActionFn(9);`
        0, // on ")", error
        -56, // on "*", reduce `SelectItem = "*" => ActionFn(9);`
        0, // on ",", error
        0, // on ".", error
        -56, // on ";", reduce `SelectItem = "*" => ActionFn(9);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -56, // on "FALSE", reduce `SelectItem = "*" => ActionFn(9);`
        -56, // on "FROM", reduce `SelectItem = "*" => ActionFn(9);`
        0, // on "INSERT INTO", error
        -56, // on "NULL", reduce `SelectItem = "*" => ActionFn(9);`
        0, // on "SELECT", error
        -56, // on "TRUE", reduce `SelectItem = "*" => ActionFn(9);`
        -56, // on "WHERE", reduce `SelectItem = "*" => ActionFn(9);`
        -56, // on "r[0-9]+", reduce `SelectItem = "*" => ActionFn(9);`
        -56, // on "r[0-9]+\\.[0-9]*", reduce `SelectItem = "*" => ActionFn(9);`
        -56, // on "r\\.[0-9]+", reduce `SelectItem = "*" => ActionFn(9);`
        -56, // on r#"\"[^\"]*\""#, reduce `SelectItem = "*" => ActionFn(9);`
        -56, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `SelectItem = "*" => ActionFn(9);`
        // State 22
        -19, // on "!=", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "(", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on ")", error
        -19, // on "*", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on ",", error
        0, // on ".", error
        -19, // on ";", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "<", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "<=", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "=", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on ">", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on ">=", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "FALSE", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "FROM", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on "INSERT INTO", error
        -19, // on "NULL", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on "SELECT", error
        -19, // on "TRUE", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "WHERE", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "r[0-9]+", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "r[0-9]+\\.[0-9]*", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "r\\.[0-9]+", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on r#"\"[^\"]*\""#, reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Boolean = "FALSE" => ActionFn(26);`
        // State 23
        -61, // on "!=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "(", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on ")", error
        -61, // on "*", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on ",", error
        0, // on ".", error
        -61, // on ";", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "<", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "<=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on ">", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on ">=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "FALSE", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "FROM", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on "INSERT INTO", error
        -61, // on "NULL", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on "SELECT", error
        -61, // on "TRUE", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "WHERE", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "r[0-9]+", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "r[0-9]+\\.[0-9]*", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "r\\.[0-9]+", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on r#"\"[^\"]*\""#, reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ValueExpression = "NULL" => ActionFn(17);`
        // State 24
        -18, // on "!=", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "(", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on ")", error
        -18, // on "*", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on ",", error
        0, // on ".", error
        -18, // on ";", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "<", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "<=", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "=", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on ">", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on ">=", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "FALSE", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "FROM", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on "INSERT INTO", error
        -18, // on "NULL", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on "SELECT", error
        -18, // on "TRUE", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "WHERE", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "r[0-9]+", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "r[0-9]+\\.[0-9]*", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "r\\.[0-9]+", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on r#"\"[^\"]*\""#, reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Boolean = "TRUE" => ActionFn(25);`
        // State 25
        -36, // on "!=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "(", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on ")", error
        -36, // on "*", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on ",", error
        0, // on ".", error
        -36, // on ";", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "<", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "<=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on ">", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on ">=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "FALSE", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "FROM", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on "INSERT INTO", error
        -36, // on "NULL", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on "SELECT", error
        -36, // on "TRUE", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "WHERE", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "r[0-9]+", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "r[0-9]+\\.[0-9]*", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "r\\.[0-9]+", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on r#"\"[^\"]*\""#, reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Number = "r[0-9]+" => ActionFn(22);`
        // State 26
        -37, // on "!=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "(", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on ")", error
        -37, // on "*", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on ",", error
        0, // on ".", error
        -37, // on ";", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "<", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "<=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on ">", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on ">=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "FALSE", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "FROM", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on "INSERT INTO", error
        -37, // on "NULL", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on "SELECT", error
        -37, // on "TRUE", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "WHERE", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "r[0-9]+", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "r[0-9]+\\.[0-9]*", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "r\\.[0-9]+", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on r#"\"[^\"]*\""#, reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        // State 27
        -38, // on "!=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "(", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on ")", error
        -38, // on "*", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on ",", error
        0, // on ".", error
        -38, // on ";", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "<", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "<=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on ">", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on ">=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "FALSE", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "FROM", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on "INSERT INTO", error
        -38, // on "NULL", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on "SELECT", error
        -38, // on "TRUE", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "WHERE", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "r[0-9]+", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "r[0-9]+\\.[0-9]*", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "r\\.[0-9]+", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on r#"\"[^\"]*\""#, reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        // State 28
        -33, // on "!=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "(", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on ")", error
        -33, // on "*", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on ",", error
        -33, // on ".", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ";", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "<", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "<=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ">", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ">=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "FALSE", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "FROM", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "INSERT INTO", error
        -33, // on "NULL", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "SELECT", error
        -33, // on "TRUE", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "WHERE", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "r[0-9]+", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "r[0-9]+\\.[0-9]*", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "r\\.[0-9]+", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on r#"\"[^\"]*\""#, reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        // State 29
        -34, // on "!=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "(", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on ")", error
        -34, // on "*", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on ",", error
        -34, // on ".", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ";", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "<", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "<=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ">", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ">=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "FALSE", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "FROM", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "INSERT INTO", error
        -34, // on "NULL", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "SELECT", error
        -34, // on "TRUE", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "WHERE", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "r[0-9]+", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "r[0-9]+\\.[0-9]*", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "r\\.[0-9]+", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on r#"\"[^\"]*\""#, reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        // State 30
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        79, // on ".", goto 78
        -52, // on ";", reduce `Relation = Identifier, ("." <Identifier>)+ => ActionFn(73);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 31
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        11, // on r#"\"[^\"]*\""#, goto 10
        12, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 11
        // State 32
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -42, // on ";", reduce `Query = "SELECT", Comma<SelectItem>, QueryFrom => ActionFn(81);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        36, // on "WHERE", goto 35
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 33
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -43, // on ";", reduce `Query = "SELECT", Comma<SelectItem>, QueryWhere => ActionFn(82);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 34
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        85, // on r#"\"[^\"]*\""#, goto 84
        86, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 85
        // State 35
        0, // on "!=", error
        93, // on "(", goto 92
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        94, // on "FALSE", goto 93
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        95, // on "NULL", goto 94
        0, // on "SELECT", error
        96, // on "TRUE", goto 95
        0, // on "WHERE", error
        97, // on "r[0-9]+", goto 96
        98, // on "r[0-9]+\\.[0-9]*", goto 97
        99, // on "r\\.[0-9]+", goto 98
        100, // on r#"\"[^\"]*\""#, goto 99
        101, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 100
        // State 36
        -65, // on "!=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "(", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on ")", error
        -65, // on "*", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on ",", error
        102, // on ".", goto 101
        -65, // on ";", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "<", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "<=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on ">", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on ">=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "FALSE", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "FROM", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on "INSERT INTO", error
        -65, // on "NULL", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on "SELECT", error
        -65, // on "TRUE", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "WHERE", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "r[0-9]+", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "r[0-9]+\\.[0-9]*", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "r\\.[0-9]+", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on r#"\"[^\"]*\""#, reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        // State 37
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        104, // on "*", goto 103
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        29, // on r#"\"[^\"]*\""#, goto 28
        30, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 29
        // State 38
        0, // on "!=", error
        47, // on "(", goto 46
        0, // on ")", error
        48, // on "*", goto 47
        0, // on ",", error
        0, // on ".", error
        -24, // on ";", reduce `Comma<SelectItem> = SelectItem, (<SelectItem> ",")+ => ActionFn(71);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        49, // on "FALSE", goto 48
        -24, // on "FROM", reduce `Comma<SelectItem> = SelectItem, (<SelectItem> ",")+ => ActionFn(71);`
        0, // on "INSERT INTO", error
        50, // on "NULL", goto 49
        0, // on "SELECT", error
        51, // on "TRUE", goto 50
        -24, // on "WHERE", reduce `Comma<SelectItem> = SelectItem, (<SelectItem> ",")+ => ActionFn(71);`
        52, // on "r[0-9]+", goto 51
        53, // on "r[0-9]+\\.[0-9]*", goto 52
        54, // on "r\\.[0-9]+", goto 53
        55, // on r#"\"[^\"]*\""#, goto 54
        56, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 55
        // State 39
        -63, // on "!=", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -63, // on ",", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on ".", error
        0, // on ";", error
        -63, // on "<", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "<=", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "=", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on ">", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on ">=", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 40
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -32, // on ",", reduce `Expression = BooleanExpression => ActionFn(15);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 41
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -53, // on ",", reduce `SelectItem = Expression => ActionFn(7);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 42
        -64, // on "!=", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -64, // on ",", reduce `ValueExpression = Identifier => ActionFn(76);`
        107, // on ".", goto 106
        0, // on ";", error
        -64, // on "<", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "<=", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "=", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on ">", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on ">=", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 43
        -62, // on "!=", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -62, // on ",", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on ".", error
        0, // on ";", error
        -62, // on "<", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "<=", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "=", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on ">", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on ">=", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 44
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        108, // on ",", goto 107
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 45
        58, // on "!=", goto 57
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -31, // on ",", reduce `Expression = ValueExpression => ActionFn(14);`
        0, // on ".", error
        0, // on ";", error
        59, // on "<", goto 58
        60, // on "<=", goto 59
        61, // on "=", goto 60
        62, // on ">", goto 61
        63, // on ">=", goto 62
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 46
        0, // on "!=", error
        70, // on "(", goto 69
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        71, // on "FALSE", goto 70
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        72, // on "NULL", goto 71
        0, // on "SELECT", error
        73, // on "TRUE", goto 72
        0, // on "WHERE", error
        74, // on "r[0-9]+", goto 73
        75, // on "r[0-9]+\\.[0-9]*", goto 74
        76, // on "r\\.[0-9]+", goto 75
        77, // on r#"\"[^\"]*\""#, goto 76
        78, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 77
        // State 47
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -56, // on ",", reduce `SelectItem = "*" => ActionFn(9);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 48
        -19, // on "!=", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -19, // on ",", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on ".", error
        0, // on ";", error
        -19, // on "<", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "<=", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "=", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on ">", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on ">=", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 49
        -61, // on "!=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -61, // on ",", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on ".", error
        0, // on ";", error
        -61, // on "<", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "<=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on ">", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on ">=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 50
        -18, // on "!=", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -18, // on ",", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on ".", error
        0, // on ";", error
        -18, // on "<", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "<=", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "=", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on ">", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on ">=", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 51
        -36, // on "!=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -36, // on ",", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on ".", error
        0, // on ";", error
        -36, // on "<", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "<=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on ">", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on ">=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 52
        -37, // on "!=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -37, // on ",", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on ".", error
        0, // on ";", error
        -37, // on "<", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "<=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on ">", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on ">=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 53
        -38, // on "!=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -38, // on ",", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on ".", error
        0, // on ";", error
        -38, // on "<", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "<=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on ">", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on ">=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 54
        -33, // on "!=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -33, // on ",", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ".", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on ";", error
        -33, // on "<", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "<=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ">", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ">=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 55
        -34, // on "!=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -34, // on ",", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ".", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on ";", error
        -34, // on "<", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "<=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ">", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ">=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 56
        0, // on "!=", error
        115, // on "(", goto 114
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        116, // on "FALSE", goto 115
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        117, // on "NULL", goto 116
        0, // on "SELECT", error
        118, // on "TRUE", goto 117
        0, // on "WHERE", error
        119, // on "r[0-9]+", goto 118
        120, // on "r[0-9]+\\.[0-9]*", goto 119
        121, // on "r\\.[0-9]+", goto 120
        122, // on r#"\"[^\"]*\""#, goto 121
        123, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 122
        // State 57
        0, // on "!=", error
        -26, // on "(", reduce `ComparisonOperator = "!=" => ActionFn(28);`
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -26, // on "FALSE", reduce `ComparisonOperator = "!=" => ActionFn(28);`
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        -26, // on "NULL", reduce `ComparisonOperator = "!=" => ActionFn(28);`
        0, // on "SELECT", error
        -26, // on "TRUE", reduce `ComparisonOperator = "!=" => ActionFn(28);`
        0, // on "WHERE", error
        -26, // on "r[0-9]+", reduce `ComparisonOperator = "!=" => ActionFn(28);`
        -26, // on "r[0-9]+\\.[0-9]*", reduce `ComparisonOperator = "!=" => ActionFn(28);`
        -26, // on "r\\.[0-9]+", reduce `ComparisonOperator = "!=" => ActionFn(28);`
        -26, // on r#"\"[^\"]*\""#, reduce `ComparisonOperator = "!=" => ActionFn(28);`
        -26, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ComparisonOperator = "!=" => ActionFn(28);`
        // State 58
        0, // on "!=", error
        -29, // on "(", reduce `ComparisonOperator = "<" => ActionFn(31);`
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -29, // on "FALSE", reduce `ComparisonOperator = "<" => ActionFn(31);`
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        -29, // on "NULL", reduce `ComparisonOperator = "<" => ActionFn(31);`
        0, // on "SELECT", error
        -29, // on "TRUE", reduce `ComparisonOperator = "<" => ActionFn(31);`
        0, // on "WHERE", error
        -29, // on "r[0-9]+", reduce `ComparisonOperator = "<" => ActionFn(31);`
        -29, // on "r[0-9]+\\.[0-9]*", reduce `ComparisonOperator = "<" => ActionFn(31);`
        -29, // on "r\\.[0-9]+", reduce `ComparisonOperator = "<" => ActionFn(31);`
        -29, // on r#"\"[^\"]*\""#, reduce `ComparisonOperator = "<" => ActionFn(31);`
        -29, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ComparisonOperator = "<" => ActionFn(31);`
        // State 59
        0, // on "!=", error
        -30, // on "(", reduce `ComparisonOperator = "<=" => ActionFn(32);`
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -30, // on "FALSE", reduce `ComparisonOperator = "<=" => ActionFn(32);`
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        -30, // on "NULL", reduce `ComparisonOperator = "<=" => ActionFn(32);`
        0, // on "SELECT", error
        -30, // on "TRUE", reduce `ComparisonOperator = "<=" => ActionFn(32);`
        0, // on "WHERE", error
        -30, // on "r[0-9]+", reduce `ComparisonOperator = "<=" => ActionFn(32);`
        -30, // on "r[0-9]+\\.[0-9]*", reduce `ComparisonOperator = "<=" => ActionFn(32);`
        -30, // on "r\\.[0-9]+", reduce `ComparisonOperator = "<=" => ActionFn(32);`
        -30, // on r#"\"[^\"]*\""#, reduce `ComparisonOperator = "<=" => ActionFn(32);`
        -30, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ComparisonOperator = "<=" => ActionFn(32);`
        // State 60
        0, // on "!=", error
        -25, // on "(", reduce `ComparisonOperator = "=" => ActionFn(27);`
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -25, // on "FALSE", reduce `ComparisonOperator = "=" => ActionFn(27);`
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        -25, // on "NULL", reduce `ComparisonOperator = "=" => ActionFn(27);`
        0, // on "SELECT", error
        -25, // on "TRUE", reduce `ComparisonOperator = "=" => ActionFn(27);`
        0, // on "WHERE", error
        -25, // on "r[0-9]+", reduce `ComparisonOperator = "=" => ActionFn(27);`
        -25, // on "r[0-9]+\\.[0-9]*", reduce `ComparisonOperator = "=" => ActionFn(27);`
        -25, // on "r\\.[0-9]+", reduce `ComparisonOperator = "=" => ActionFn(27);`
        -25, // on r#"\"[^\"]*\""#, reduce `ComparisonOperator = "=" => ActionFn(27);`
        -25, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ComparisonOperator = "=" => ActionFn(27);`
        // State 61
        0, // on "!=", error
        -27, // on "(", reduce `ComparisonOperator = ">" => ActionFn(29);`
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -27, // on "FALSE", reduce `ComparisonOperator = ">" => ActionFn(29);`
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        -27, // on "NULL", reduce `ComparisonOperator = ">" => ActionFn(29);`
        0, // on "SELECT", error
        -27, // on "TRUE", reduce `ComparisonOperator = ">" => ActionFn(29);`
        0, // on "WHERE", error
        -27, // on "r[0-9]+", reduce `ComparisonOperator = ">" => ActionFn(29);`
        -27, // on "r[0-9]+\\.[0-9]*", reduce `ComparisonOperator = ">" => ActionFn(29);`
        -27, // on "r\\.[0-9]+", reduce `ComparisonOperator = ">" => ActionFn(29);`
        -27, // on r#"\"[^\"]*\""#, reduce `ComparisonOperator = ">" => ActionFn(29);`
        -27, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ComparisonOperator = ">" => ActionFn(29);`
        // State 62
        0, // on "!=", error
        -28, // on "(", reduce `ComparisonOperator = ">=" => ActionFn(30);`
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -28, // on "FALSE", reduce `ComparisonOperator = ">=" => ActionFn(30);`
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        -28, // on "NULL", reduce `ComparisonOperator = ">=" => ActionFn(30);`
        0, // on "SELECT", error
        -28, // on "TRUE", reduce `ComparisonOperator = ">=" => ActionFn(30);`
        0, // on "WHERE", error
        -28, // on "r[0-9]+", reduce `ComparisonOperator = ">=" => ActionFn(30);`
        -28, // on "r[0-9]+\\.[0-9]*", reduce `ComparisonOperator = ">=" => ActionFn(30);`
        -28, // on "r\\.[0-9]+", reduce `ComparisonOperator = ">=" => ActionFn(30);`
        -28, // on r#"\"[^\"]*\""#, reduce `ComparisonOperator = ">=" => ActionFn(30);`
        -28, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ComparisonOperator = ">=" => ActionFn(30);`
        // State 63
        -63, // on "!=", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on "(", error
        -63, // on ")", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        -63, // on "<", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "<=", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "=", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on ">", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on ">=", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 64
        0, // on "!=", error
        0, // on "(", error
        -32, // on ")", reduce `Expression = BooleanExpression => ActionFn(15);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 65
        0, // on "!=", error
        0, // on "(", error
        124, // on ")", goto 123
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 66
        -64, // on "!=", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on "(", error
        -64, // on ")", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on "*", error
        0, // on ",", error
        126, // on ".", goto 125
        0, // on ";", error
        -64, // on "<", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "<=", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "=", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on ">", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on ">=", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 67
        -62, // on "!=", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on "(", error
        -62, // on ")", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        -62, // on "<", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "<=", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "=", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on ">", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on ">=", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 68
        58, // on "!=", goto 57
        0, // on "(", error
        -31, // on ")", reduce `Expression = ValueExpression => ActionFn(14);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        59, // on "<", goto 58
        60, // on "<=", goto 59
        61, // on "=", goto 60
        62, // on ">", goto 61
        63, // on ">=", goto 62
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 69
        0, // on "!=", error
        70, // on "(", goto 69
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        71, // on "FALSE", goto 70
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        72, // on "NULL", goto 71
        0, // on "SELECT", error
        73, // on "TRUE", goto 72
        0, // on "WHERE", error
        74, // on "r[0-9]+", goto 73
        75, // on "r[0-9]+\\.[0-9]*", goto 74
        76, // on "r\\.[0-9]+", goto 75
        77, // on r#"\"[^\"]*\""#, goto 76
        78, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 77
        // State 70
        -19, // on "!=", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on "(", error
        -19, // on ")", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        -19, // on "<", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "<=", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "=", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on ">", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on ">=", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 71
        -61, // on "!=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on "(", error
        -61, // on ")", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        -61, // on "<", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "<=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on ">", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on ">=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 72
        -18, // on "!=", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on "(", error
        -18, // on ")", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        -18, // on "<", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "<=", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "=", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on ">", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on ">=", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 73
        -36, // on "!=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on "(", error
        -36, // on ")", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        -36, // on "<", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "<=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on ">", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on ">=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 74
        -37, // on "!=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on "(", error
        -37, // on ")", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        -37, // on "<", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "<=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on ">", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on ">=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 75
        -38, // on "!=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on "(", error
        -38, // on ")", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        -38, // on "<", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "<=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on ">", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on ">=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 76
        -33, // on "!=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "(", error
        -33, // on ")", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "*", error
        0, // on ",", error
        -33, // on ".", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on ";", error
        -33, // on "<", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "<=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ">", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ">=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 77
        -34, // on "!=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "(", error
        -34, // on ")", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "*", error
        0, // on ",", error
        -34, // on ".", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on ";", error
        -34, // on "<", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "<=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ">", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ">=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 78
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        11, // on r#"\"[^\"]*\""#, goto 10
        12, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 11
        // State 79
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        -6, // on ".", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ";", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 80
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -41, // on ";", reduce `Query = "SELECT", Comma<SelectItem>, QueryFrom, QueryWhere => ActionFn(80);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 81
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -45, // on ";", reduce `QueryFrom = "FROM", Comma<Relation> => ActionFn(4);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        -45, // on "WHERE", reduce `QueryFrom = "FROM", Comma<Relation> => ActionFn(4);`
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 82
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        131, // on ".", goto 130
        -51, // on ";", reduce `Relation = Identifier => ActionFn(72);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        -51, // on "WHERE", reduce `Relation = Identifier => ActionFn(72);`
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        -51, // on r#"\"[^\"]*\""#, reduce `Relation = Identifier => ActionFn(72);`
        -51, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Relation = Identifier => ActionFn(72);`
        // State 83
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -21, // on ";", reduce `Comma<Relation> = Relation => ActionFn(66);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        -21, // on "WHERE", reduce `Comma<Relation> = Relation => ActionFn(66);`
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        135, // on r#"\"[^\"]*\""#, goto 134
        136, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 135
        // State 84
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        -33, // on ".", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ";", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        -33, // on "WHERE", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        -33, // on r#"\"[^\"]*\""#, reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        // State 85
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        -34, // on ".", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ";", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        -34, // on "WHERE", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        -34, // on r#"\"[^\"]*\""#, reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        // State 86
        -63, // on "!=", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -63, // on ";", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "<", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "<=", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "=", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on ">", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on ">=", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 87
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -32, // on ";", reduce `Expression = BooleanExpression => ActionFn(15);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 88
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -48, // on ";", reduce `QueryWhere = "WHERE", Expression => ActionFn(5);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 89
        -64, // on "!=", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        138, // on ".", goto 137
        -64, // on ";", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "<", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "<=", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "=", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on ">", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on ">=", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 90
        -62, // on "!=", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -62, // on ";", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "<", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "<=", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "=", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on ">", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on ">=", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 91
        58, // on "!=", goto 57
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -31, // on ";", reduce `Expression = ValueExpression => ActionFn(14);`
        59, // on "<", goto 58
        60, // on "<=", goto 59
        61, // on "=", goto 60
        62, // on ">", goto 61
        63, // on ">=", goto 62
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 92
        0, // on "!=", error
        70, // on "(", goto 69
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        71, // on "FALSE", goto 70
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        72, // on "NULL", goto 71
        0, // on "SELECT", error
        73, // on "TRUE", goto 72
        0, // on "WHERE", error
        74, // on "r[0-9]+", goto 73
        75, // on "r[0-9]+\\.[0-9]*", goto 74
        76, // on "r\\.[0-9]+", goto 75
        77, // on r#"\"[^\"]*\""#, goto 76
        78, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 77
        // State 93
        -19, // on "!=", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -19, // on ";", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "<", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "<=", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "=", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on ">", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on ">=", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 94
        -61, // on "!=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -61, // on ";", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "<", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "<=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on ">", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on ">=", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 95
        -18, // on "!=", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -18, // on ";", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "<", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "<=", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "=", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on ">", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on ">=", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 96
        -36, // on "!=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -36, // on ";", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "<", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "<=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on ">", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on ">=", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 97
        -37, // on "!=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -37, // on ";", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "<", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "<=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on ">", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on ">=", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 98
        -38, // on "!=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -38, // on ";", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "<", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "<=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on ">", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on ">=", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 99
        -33, // on "!=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        -33, // on ".", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ";", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "<", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "<=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ">", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ">=", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 100
        -34, // on "!=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        -34, // on ".", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ";", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "<", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "<=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ">", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ">=", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 101
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        142, // on "*", goto 141
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        29, // on r#"\"[^\"]*\""#, goto 28
        30, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 29
        // State 102
        -6, // on "!=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "(", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on ")", error
        -6, // on "*", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on ",", error
        -6, // on ".", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ";", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "<", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "<=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ">", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ">=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "FALSE", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "FROM", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "INSERT INTO", error
        -6, // on "NULL", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "SELECT", error
        -6, // on "TRUE", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "WHERE", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "r[0-9]+", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "r[0-9]+\\.[0-9]*", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "r\\.[0-9]+", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on r#"\"[^\"]*\""#, reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        // State 103
        0, // on "!=", error
        -54, // on "(", reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        0, // on ")", error
        -54, // on "*", reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        0, // on ",", error
        0, // on ".", error
        -54, // on ";", reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -54, // on "FALSE", reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        -54, // on "FROM", reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        0, // on "INSERT INTO", error
        -54, // on "NULL", reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        0, // on "SELECT", error
        -54, // on "TRUE", reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        -54, // on "WHERE", reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        -54, // on "r[0-9]+", reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        -54, // on "r[0-9]+\\.[0-9]*", reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        -54, // on "r\\.[0-9]+", reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        -54, // on r#"\"[^\"]*\""#, reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        -54, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        // State 104
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        143, // on ",", goto 142
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 105
        -65, // on "!=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -65, // on ",", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        144, // on ".", goto 143
        0, // on ";", error
        -65, // on "<", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "<=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on ">", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on ">=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 106
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        146, // on "*", goto 145
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        55, // on r#"\"[^\"]*\""#, goto 54
        56, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 55
        // State 107
        0, // on "!=", error
        -16, // on "(", reduce `(<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);`
        0, // on ")", error
        -16, // on "*", reduce `(<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);`
        0, // on ",", error
        0, // on ".", error
        -16, // on ";", reduce `(<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -16, // on "FALSE", reduce `(<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);`
        -16, // on "FROM", reduce `(<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);`
        0, // on "INSERT INTO", error
        -16, // on "NULL", reduce `(<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);`
        0, // on "SELECT", error
        -16, // on "TRUE", reduce `(<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);`
        -16, // on "WHERE", reduce `(<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);`
        -16, // on "r[0-9]+", reduce `(<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);`
        -16, // on "r[0-9]+\\.[0-9]*", reduce `(<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);`
        -16, // on "r\\.[0-9]+", reduce `(<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);`
        -16, // on r#"\"[^\"]*\""#, reduce `(<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);`
        -16, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `(<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);`
        // State 108
        0, // on "!=", error
        151, // on "(", goto 150
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        152, // on "FALSE", goto 151
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        153, // on "NULL", goto 152
        0, // on "SELECT", error
        154, // on "TRUE", goto 153
        0, // on "WHERE", error
        155, // on "r[0-9]+", goto 154
        156, // on "r[0-9]+\\.[0-9]*", goto 155
        157, // on "r\\.[0-9]+", goto 156
        135, // on r#"\"[^\"]*\""#, goto 134
        136, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 135
        // State 109
        0, // on "!=", error
        0, // on "(", error
        158, // on ")", goto 157
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 110
        0, // on "!=", error
        -63, // on "(", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on ")", error
        -63, // on "*", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on ",", error
        0, // on ".", error
        -63, // on ";", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -63, // on "FALSE", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "FROM", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on "INSERT INTO", error
        -63, // on "NULL", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on "SELECT", error
        -63, // on "TRUE", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "WHERE", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "r[0-9]+", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "r[0-9]+\\.[0-9]*", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on "r\\.[0-9]+", reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on r#"\"[^\"]*\""#, reduce `ValueExpression = Boolean => ActionFn(19);`
        -63, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ValueExpression = Boolean => ActionFn(19);`
        // State 111
        0, // on "!=", error
        -64, // on "(", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on ")", error
        -64, // on "*", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on ",", error
        160, // on ".", goto 159
        -64, // on ";", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -64, // on "FALSE", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "FROM", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on "INSERT INTO", error
        -64, // on "NULL", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on "SELECT", error
        -64, // on "TRUE", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "WHERE", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "r[0-9]+", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "r[0-9]+\\.[0-9]*", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on "r\\.[0-9]+", reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on r#"\"[^\"]*\""#, reduce `ValueExpression = Identifier => ActionFn(76);`
        -64, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ValueExpression = Identifier => ActionFn(76);`
        // State 112
        0, // on "!=", error
        -62, // on "(", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on ")", error
        -62, // on "*", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on ",", error
        0, // on ".", error
        -62, // on ";", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -62, // on "FALSE", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "FROM", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on "INSERT INTO", error
        -62, // on "NULL", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on "SELECT", error
        -62, // on "TRUE", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "WHERE", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "r[0-9]+", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "r[0-9]+\\.[0-9]*", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on "r\\.[0-9]+", reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on r#"\"[^\"]*\""#, reduce `ValueExpression = Number => ActionFn(18);`
        -62, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ValueExpression = Number => ActionFn(18);`
        // State 113
        0, // on "!=", error
        -20, // on "(", reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        0, // on ")", error
        -20, // on "*", reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        0, // on ",", error
        0, // on ".", error
        -20, // on ";", reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -20, // on "FALSE", reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        -20, // on "FROM", reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        0, // on "INSERT INTO", error
        -20, // on "NULL", reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        0, // on "SELECT", error
        -20, // on "TRUE", reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        -20, // on "WHERE", reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        -20, // on "r[0-9]+", reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        -20, // on "r[0-9]+\\.[0-9]*", reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        -20, // on "r\\.[0-9]+", reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        -20, // on r#"\"[^\"]*\""#, reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        -20, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        // State 114
        0, // on "!=", error
        70, // on "(", goto 69
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        71, // on "FALSE", goto 70
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        72, // on "NULL", goto 71
        0, // on "SELECT", error
        73, // on "TRUE", goto 72
        0, // on "WHERE", error
        74, // on "r[0-9]+", goto 73
        75, // on "r[0-9]+\\.[0-9]*", goto 74
        76, // on "r\\.[0-9]+", goto 75
        77, // on r#"\"[^\"]*\""#, goto 76
        78, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 77
        // State 115
        0, // on "!=", error
        -19, // on "(", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on ")", error
        -19, // on "*", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on ",", error
        0, // on ".", error
        -19, // on ";", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -19, // on "FALSE", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "FROM", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on "INSERT INTO", error
        -19, // on "NULL", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on "SELECT", error
        -19, // on "TRUE", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "WHERE", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "r[0-9]+", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "r[0-9]+\\.[0-9]*", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on "r\\.[0-9]+", reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on r#"\"[^\"]*\""#, reduce `Boolean = "FALSE" => ActionFn(26);`
        -19, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Boolean = "FALSE" => ActionFn(26);`
        // State 116
        0, // on "!=", error
        -61, // on "(", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on ")", error
        -61, // on "*", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on ",", error
        0, // on ".", error
        -61, // on ";", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -61, // on "FALSE", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "FROM", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on "INSERT INTO", error
        -61, // on "NULL", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on "SELECT", error
        -61, // on "TRUE", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "WHERE", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "r[0-9]+", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "r[0-9]+\\.[0-9]*", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on "r\\.[0-9]+", reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on r#"\"[^\"]*\""#, reduce `ValueExpression = "NULL" => ActionFn(17);`
        -61, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ValueExpression = "NULL" => ActionFn(17);`
        // State 117
        0, // on "!=", error
        -18, // on "(", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on ")", error
        -18, // on "*", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on ",", error
        0, // on ".", error
        -18, // on ";", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -18, // on "FALSE", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "FROM", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on "INSERT INTO", error
        -18, // on "NULL", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on "SELECT", error
        -18, // on "TRUE", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "WHERE", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "r[0-9]+", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "r[0-9]+\\.[0-9]*", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on "r\\.[0-9]+", reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on r#"\"[^\"]*\""#, reduce `Boolean = "TRUE" => ActionFn(25);`
        -18, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Boolean = "TRUE" => ActionFn(25);`
        // State 118
        0, // on "!=", error
        -36, // on "(", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on ")", error
        -36, // on "*", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on ",", error
        0, // on ".", error
        -36, // on ";", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -36, // on "FALSE", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "FROM", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on "INSERT INTO", error
        -36, // on "NULL", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on "SELECT", error
        -36, // on "TRUE", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "WHERE", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "r[0-9]+", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "r[0-9]+\\.[0-9]*", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on "r\\.[0-9]+", reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on r#"\"[^\"]*\""#, reduce `Number = "r[0-9]+" => ActionFn(22);`
        -36, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Number = "r[0-9]+" => ActionFn(22);`
        // State 119
        0, // on "!=", error
        -37, // on "(", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on ")", error
        -37, // on "*", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on ",", error
        0, // on ".", error
        -37, // on ";", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -37, // on "FALSE", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "FROM", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on "INSERT INTO", error
        -37, // on "NULL", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on "SELECT", error
        -37, // on "TRUE", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "WHERE", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "r[0-9]+", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "r[0-9]+\\.[0-9]*", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on "r\\.[0-9]+", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on r#"\"[^\"]*\""#, reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -37, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        // State 120
        0, // on "!=", error
        -38, // on "(", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on ")", error
        -38, // on "*", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on ",", error
        0, // on ".", error
        -38, // on ";", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -38, // on "FALSE", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "FROM", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on "INSERT INTO", error
        -38, // on "NULL", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on "SELECT", error
        -38, // on "TRUE", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "WHERE", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "r[0-9]+", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "r[0-9]+\\.[0-9]*", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on "r\\.[0-9]+", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on r#"\"[^\"]*\""#, reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -38, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        // State 121
        0, // on "!=", error
        -33, // on "(", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on ")", error
        -33, // on "*", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on ",", error
        -33, // on ".", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ";", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -33, // on "FALSE", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "FROM", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "INSERT INTO", error
        -33, // on "NULL", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "SELECT", error
        -33, // on "TRUE", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "WHERE", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "r[0-9]+", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "r[0-9]+\\.[0-9]*", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on "r\\.[0-9]+", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on r#"\"[^\"]*\""#, reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        // State 122
        0, // on "!=", error
        -34, // on "(", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on ")", error
        -34, // on "*", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on ",", error
        -34, // on ".", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ";", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -34, // on "FALSE", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "FROM", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "INSERT INTO", error
        -34, // on "NULL", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "SELECT", error
        -34, // on "TRUE", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "WHERE", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "r[0-9]+", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "r[0-9]+\\.[0-9]*", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on "r\\.[0-9]+", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on r#"\"[^\"]*\""#, reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        // State 123
        -66, // on "!=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "(", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on ")", error
        -66, // on "*", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on ",", error
        0, // on ".", error
        -66, // on ";", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "<", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "<=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on ">", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on ">=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "FALSE", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "FROM", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on "INSERT INTO", error
        -66, // on "NULL", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on "SELECT", error
        -66, // on "TRUE", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "WHERE", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "r[0-9]+", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "r[0-9]+\\.[0-9]*", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "r\\.[0-9]+", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on r#"\"[^\"]*\""#, reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        // State 124
        -65, // on "!=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on "(", error
        -65, // on ")", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on "*", error
        0, // on ",", error
        162, // on ".", goto 161
        0, // on ";", error
        -65, // on "<", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "<=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on ">", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on ">=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 125
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        77, // on r#"\"[^\"]*\""#, goto 76
        78, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 77
        // State 126
        0, // on "!=", error
        168, // on "(", goto 167
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        169, // on "FALSE", goto 168
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        170, // on "NULL", goto 169
        0, // on "SELECT", error
        171, // on "TRUE", goto 170
        0, // on "WHERE", error
        172, // on "r[0-9]+", goto 171
        173, // on "r[0-9]+\\.[0-9]*", goto 172
        174, // on "r\\.[0-9]+", goto 173
        175, // on r#"\"[^\"]*\""#, goto 174
        176, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 175
        // State 127
        0, // on "!=", error
        0, // on "(", error
        177, // on ")", goto 176
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 128
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        -7, // on ".", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ";", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 129
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        178, // on ".", goto 177
        -52, // on ";", reduce `Relation = Identifier, ("." <Identifier>)+ => ActionFn(73);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        -52, // on "WHERE", reduce `Relation = Identifier, ("." <Identifier>)+ => ActionFn(73);`
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        -52, // on r#"\"[^\"]*\""#, reduce `Relation = Identifier, ("." <Identifier>)+ => ActionFn(73);`
        -52, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `Relation = Identifier, ("." <Identifier>)+ => ActionFn(73);`
        // State 130
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        85, // on r#"\"[^\"]*\""#, goto 84
        86, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 85
        // State 131
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -22, // on ";", reduce `Comma<Relation> = Relation, (<Relation> ",")+ => ActionFn(67);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        -22, // on "WHERE", reduce `Comma<Relation> = Relation, (<Relation> ",")+ => ActionFn(67);`
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        135, // on r#"\"[^\"]*\""#, goto 134
        136, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 135
        // State 132
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -51, // on ",", reduce `Relation = Identifier => ActionFn(72);`
        182, // on ".", goto 181
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 133
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        183, // on ",", goto 182
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 134
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -33, // on ",", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -33, // on ".", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 135
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -34, // on ",", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -34, // on ".", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 136
        -65, // on "!=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        184, // on ".", goto 183
        -65, // on ";", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "<", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "<=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on ">", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on ">=", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 137
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        100, // on r#"\"[^\"]*\""#, goto 99
        101, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 100
        // State 138
        0, // on "!=", error
        190, // on "(", goto 189
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        191, // on "FALSE", goto 190
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        192, // on "NULL", goto 191
        0, // on "SELECT", error
        193, // on "TRUE", goto 192
        0, // on "WHERE", error
        194, // on "r[0-9]+", goto 193
        195, // on "r[0-9]+\\.[0-9]*", goto 194
        196, // on "r\\.[0-9]+", goto 195
        11, // on r#"\"[^\"]*\""#, goto 10
        12, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 11
        // State 139
        0, // on "!=", error
        0, // on "(", error
        197, // on ")", goto 196
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 140
        -7, // on "!=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "(", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on ")", error
        -7, // on "*", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on ",", error
        -7, // on ".", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ";", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "<", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "<=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ">", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ">=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "FALSE", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "FROM", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "INSERT INTO", error
        -7, // on "NULL", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "SELECT", error
        -7, // on "TRUE", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "WHERE", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "r[0-9]+", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "r[0-9]+\\.[0-9]*", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "r\\.[0-9]+", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on r#"\"[^\"]*\""#, reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        // State 141
        0, // on "!=", error
        -55, // on "(", reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        0, // on ")", error
        -55, // on "*", reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        0, // on ",", error
        0, // on ".", error
        -55, // on ";", reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -55, // on "FALSE", reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        -55, // on "FROM", reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        0, // on "INSERT INTO", error
        -55, // on "NULL", reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        0, // on "SELECT", error
        -55, // on "TRUE", reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        -55, // on "WHERE", reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        -55, // on "r[0-9]+", reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        -55, // on "r[0-9]+\\.[0-9]*", reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        -55, // on "r\\.[0-9]+", reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        -55, // on r#"\"[^\"]*\""#, reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        -55, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        // State 142
        0, // on "!=", error
        -17, // on "(", reduce `(<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);`
        0, // on ")", error
        -17, // on "*", reduce `(<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);`
        0, // on ",", error
        0, // on ".", error
        -17, // on ";", reduce `(<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -17, // on "FALSE", reduce `(<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);`
        -17, // on "FROM", reduce `(<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);`
        0, // on "INSERT INTO", error
        -17, // on "NULL", reduce `(<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);`
        0, // on "SELECT", error
        -17, // on "TRUE", reduce `(<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);`
        -17, // on "WHERE", reduce `(<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);`
        -17, // on "r[0-9]+", reduce `(<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);`
        -17, // on "r[0-9]+\\.[0-9]*", reduce `(<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);`
        -17, // on "r\\.[0-9]+", reduce `(<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);`
        -17, // on r#"\"[^\"]*\""#, reduce `(<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);`
        -17, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `(<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);`
        // State 143
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        199, // on "*", goto 198
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        55, // on r#"\"[^\"]*\""#, goto 54
        56, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 55
        // State 144
        -6, // on "!=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -6, // on ",", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ".", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on ";", error
        -6, // on "<", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "<=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ">", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ">=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 145
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -54, // on ",", reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 146
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -63, // on ",", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 147
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -64, // on ",", reduce `ValueExpression = Identifier => ActionFn(76);`
        182, // on ".", goto 181
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 148
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -62, // on ",", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 149
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -20, // on ",", reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 150
        0, // on "!=", error
        70, // on "(", goto 69
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        71, // on "FALSE", goto 70
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        72, // on "NULL", goto 71
        0, // on "SELECT", error
        73, // on "TRUE", goto 72
        0, // on "WHERE", error
        74, // on "r[0-9]+", goto 73
        75, // on "r[0-9]+\\.[0-9]*", goto 74
        76, // on "r\\.[0-9]+", goto 75
        77, // on r#"\"[^\"]*\""#, goto 76
        78, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 77
        // State 151
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -19, // on ",", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 152
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -61, // on ",", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 153
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -18, // on ",", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 154
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -36, // on ",", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 155
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -37, // on ",", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 156
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -38, // on ",", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 157
        -66, // on "!=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -66, // on ",", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on ".", error
        0, // on ";", error
        -66, // on "<", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "<=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on ">", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on ">=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 158
        0, // on "!=", error
        -65, // on "(", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on ")", error
        -65, // on "*", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on ",", error
        202, // on ".", goto 201
        -65, // on ";", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -65, // on "FALSE", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "FROM", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on "INSERT INTO", error
        -65, // on "NULL", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on "SELECT", error
        -65, // on "TRUE", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "WHERE", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "r[0-9]+", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "r[0-9]+\\.[0-9]*", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on "r\\.[0-9]+", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on r#"\"[^\"]*\""#, reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        -65, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        // State 159
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        122, // on r#"\"[^\"]*\""#, goto 121
        123, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 122
        // State 160
        0, // on "!=", error
        0, // on "(", error
        204, // on ")", goto 203
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 161
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        77, // on r#"\"[^\"]*\""#, goto 76
        78, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 77
        // State 162
        -6, // on "!=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "(", error
        -6, // on ")", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "*", error
        0, // on ",", error
        -6, // on ".", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on ";", error
        -6, // on "<", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "<=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ">", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ">=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 163
        0, // on "!=", error
        0, // on "(", error
        -63, // on ")", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 164
        0, // on "!=", error
        0, // on "(", error
        -64, // on ")", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on "*", error
        0, // on ",", error
        207, // on ".", goto 206
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 165
        0, // on "!=", error
        0, // on "(", error
        -62, // on ")", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 166
        0, // on "!=", error
        0, // on "(", error
        -20, // on ")", reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 167
        0, // on "!=", error
        70, // on "(", goto 69
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        71, // on "FALSE", goto 70
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        72, // on "NULL", goto 71
        0, // on "SELECT", error
        73, // on "TRUE", goto 72
        0, // on "WHERE", error
        74, // on "r[0-9]+", goto 73
        75, // on "r[0-9]+\\.[0-9]*", goto 74
        76, // on "r\\.[0-9]+", goto 75
        77, // on r#"\"[^\"]*\""#, goto 76
        78, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 77
        // State 168
        0, // on "!=", error
        0, // on "(", error
        -19, // on ")", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 169
        0, // on "!=", error
        0, // on "(", error
        -61, // on ")", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 170
        0, // on "!=", error
        0, // on "(", error
        -18, // on ")", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 171
        0, // on "!=", error
        0, // on "(", error
        -36, // on ")", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 172
        0, // on "!=", error
        0, // on "(", error
        -37, // on ")", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 173
        0, // on "!=", error
        0, // on "(", error
        -38, // on ")", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 174
        0, // on "!=", error
        0, // on "(", error
        -33, // on ")", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on "*", error
        0, // on ",", error
        -33, // on ".", reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 175
        0, // on "!=", error
        0, // on "(", error
        -34, // on ")", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on "*", error
        0, // on ",", error
        -34, // on ".", reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 176
        -66, // on "!=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on "(", error
        -66, // on ")", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        -66, // on "<", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "<=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on ">", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on ">=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 177
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        85, // on r#"\"[^\"]*\""#, goto 84
        86, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 85
        // State 178
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        -6, // on ".", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ";", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        -6, // on "WHERE", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        -6, // on r#"\"[^\"]*\""#, reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        // State 179
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        210, // on ",", goto 209
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 180
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -52, // on ",", reduce `Relation = Identifier, ("." <Identifier>)+ => ActionFn(73);`
        211, // on ".", goto 210
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 181
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        135, // on r#"\"[^\"]*\""#, goto 134
        136, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 135
        // State 182
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -11, // on ";", reduce `(<Relation> ",")+ = Relation, "," => ActionFn(64);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        -11, // on "WHERE", reduce `(<Relation> ",")+ = Relation, "," => ActionFn(64);`
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        -11, // on r#"\"[^\"]*\""#, reduce `(<Relation> ",")+ = Relation, "," => ActionFn(64);`
        -11, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `(<Relation> ",")+ = Relation, "," => ActionFn(64);`
        // State 183
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        100, // on r#"\"[^\"]*\""#, goto 99
        101, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 100
        // State 184
        -6, // on "!=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        -6, // on ".", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ";", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "<", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "<=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ">", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ">=", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 185
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -63, // on ";", reduce `ValueExpression = Boolean => ActionFn(19);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 186
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        32, // on ".", goto 31
        -64, // on ";", reduce `ValueExpression = Identifier => ActionFn(76);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 187
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -62, // on ";", reduce `ValueExpression = Number => ActionFn(18);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 188
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -20, // on ";", reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 189
        0, // on "!=", error
        70, // on "(", goto 69
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        71, // on "FALSE", goto 70
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        72, // on "NULL", goto 71
        0, // on "SELECT", error
        73, // on "TRUE", goto 72
        0, // on "WHERE", error
        74, // on "r[0-9]+", goto 73
        75, // on "r[0-9]+\\.[0-9]*", goto 74
        76, // on "r\\.[0-9]+", goto 75
        77, // on r#"\"[^\"]*\""#, goto 76
        78, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 77
        // State 190
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -19, // on ";", reduce `Boolean = "FALSE" => ActionFn(26);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 191
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -61, // on ";", reduce `ValueExpression = "NULL" => ActionFn(17);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 192
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -18, // on ";", reduce `Boolean = "TRUE" => ActionFn(25);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 193
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -36, // on ";", reduce `Number = "r[0-9]+" => ActionFn(22);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 194
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -37, // on ";", reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 195
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -38, // on ";", reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 196
        -66, // on "!=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -66, // on ";", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "<", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "<=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on ">", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on ">=", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 197
        -7, // on "!=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -7, // on ",", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ".", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on ";", error
        -7, // on "<", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "<=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ">", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ">=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 198
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -55, // on ",", reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 199
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -65, // on ",", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        211, // on ".", goto 210
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 200
        0, // on "!=", error
        0, // on "(", error
        216, // on ")", goto 215
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 201
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        122, // on r#"\"[^\"]*\""#, goto 121
        123, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 122
        // State 202
        0, // on "!=", error
        -6, // on "(", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on ")", error
        -6, // on "*", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on ",", error
        -6, // on ".", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ";", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -6, // on "FALSE", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "FROM", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "INSERT INTO", error
        -6, // on "NULL", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "SELECT", error
        -6, // on "TRUE", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "WHERE", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "r[0-9]+", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "r[0-9]+\\.[0-9]*", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on "r\\.[0-9]+", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on r#"\"[^\"]*\""#, reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        // State 203
        0, // on "!=", error
        -66, // on "(", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on ")", error
        -66, // on "*", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on ",", error
        0, // on ".", error
        -66, // on ";", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -66, // on "FALSE", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "FROM", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on "INSERT INTO", error
        -66, // on "NULL", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on "SELECT", error
        -66, // on "TRUE", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "WHERE", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "r[0-9]+", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "r[0-9]+\\.[0-9]*", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on "r\\.[0-9]+", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on r#"\"[^\"]*\""#, reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        -66, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        // State 204
        -7, // on "!=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "(", error
        -7, // on ")", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "*", error
        0, // on ",", error
        -7, // on ".", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on ";", error
        -7, // on "<", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "<=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ">", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ">=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 205
        0, // on "!=", error
        0, // on "(", error
        -65, // on ")", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on "*", error
        0, // on ",", error
        218, // on ".", goto 217
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 206
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        175, // on r#"\"[^\"]*\""#, goto 174
        176, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 175
        // State 207
        0, // on "!=", error
        0, // on "(", error
        220, // on ")", goto 219
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 208
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        -7, // on ".", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ";", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        -7, // on "WHERE", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        -7, // on r#"\"[^\"]*\""#, reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        // State 209
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -12, // on ";", reduce `(<Relation> ",")+ = (<Relation> ",")+, Relation, "," => ActionFn(65);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        -12, // on "WHERE", reduce `(<Relation> ",")+ = (<Relation> ",")+, Relation, "," => ActionFn(65);`
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        -12, // on r#"\"[^\"]*\""#, reduce `(<Relation> ",")+ = (<Relation> ",")+, Relation, "," => ActionFn(65);`
        -12, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `(<Relation> ",")+ = (<Relation> ",")+, Relation, "," => ActionFn(65);`
        // State 210
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        135, // on r#"\"[^\"]*\""#, goto 134
        136, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 135
        // State 211
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -6, // on ",", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -6, // on ".", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 212
        -7, // on "!=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        -7, // on ".", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ";", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "<", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "<=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ">", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ">=", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 213
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        79, // on ".", goto 78
        -65, // on ";", reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 214
        0, // on "!=", error
        0, // on "(", error
        222, // on ")", goto 221
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 215
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -66, // on ",", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 216
        0, // on "!=", error
        -7, // on "(", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on ")", error
        -7, // on "*", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on ",", error
        -7, // on ".", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ";", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -7, // on "FALSE", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "FROM", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "INSERT INTO", error
        -7, // on "NULL", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "SELECT", error
        -7, // on "TRUE", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "WHERE", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "r[0-9]+", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "r[0-9]+\\.[0-9]*", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on "r\\.[0-9]+", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on r#"\"[^\"]*\""#, reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        // State 217
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        175, // on r#"\"[^\"]*\""#, goto 174
        176, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, goto 175
        // State 218
        0, // on "!=", error
        0, // on "(", error
        -6, // on ")", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on "*", error
        0, // on ",", error
        -6, // on ".", reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 219
        0, // on "!=", error
        0, // on "(", error
        -66, // on ")", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 220
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        -7, // on ",", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -7, // on ".", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 221
        0, // on "!=", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on ",", error
        0, // on ".", error
        -66, // on ";", reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
        // State 222
        0, // on "!=", error
        0, // on "(", error
        -7, // on ")", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on "*", error
        0, // on ",", error
        -7, // on ".", reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on ";", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "FALSE", error
        0, // on "FROM", error
        0, // on "INSERT INTO", error
        0, // on "NULL", error
        0, // on "SELECT", error
        0, // on "TRUE", error
        0, // on "WHERE", error
        0, // on "r[0-9]+", error
        0, // on "r[0-9]+\\.[0-9]*", error
        0, // on "r\\.[0-9]+", error
        0, // on r#"\"[^\"]*\""#, error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9@:_]*"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -60, // on EOF, reduce `Statement = Insert => ActionFn(59);`
        -58, // on EOF, reduce `Statement = Query => ActionFn(57);`
        -67, // on EOF, reduce `__Statement = Statement => ActionFn(0);`
        0, // on EOF, error
        0, // on EOF, error
        -59, // on EOF, reduce `Statement = Insert, ";" => ActionFn(58);`
        -57, // on EOF, reduce `Statement = Query, ";" => ActionFn(56);`
        -51, // on EOF, reduce `Relation = Identifier => ActionFn(72);`
        -35, // on EOF, reduce `Insert = "INSERT INTO", Relation => ActionFn(6);`
        -33, // on EOF, reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -34, // on EOF, reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -63, // on EOF, reduce `ValueExpression = Boolean => ActionFn(19);`
        -32, // on EOF, reduce `Expression = BooleanExpression => ActionFn(15);`
        -44, // on EOF, reduce `Query = "SELECT", Comma<SelectItem> => ActionFn(83);`
        -53, // on EOF, reduce `SelectItem = Expression => ActionFn(7);`
        -64, // on EOF, reduce `ValueExpression = Identifier => ActionFn(76);`
        -62, // on EOF, reduce `ValueExpression = Number => ActionFn(18);`
        -23, // on EOF, reduce `Comma<SelectItem> = SelectItem => ActionFn(70);`
        -31, // on EOF, reduce `Expression = ValueExpression => ActionFn(14);`
        0, // on EOF, error
        -56, // on EOF, reduce `SelectItem = "*" => ActionFn(9);`
        -19, // on EOF, reduce `Boolean = "FALSE" => ActionFn(26);`
        -61, // on EOF, reduce `ValueExpression = "NULL" => ActionFn(17);`
        -18, // on EOF, reduce `Boolean = "TRUE" => ActionFn(25);`
        -36, // on EOF, reduce `Number = "r[0-9]+" => ActionFn(22);`
        -37, // on EOF, reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -38, // on EOF, reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -33, // on EOF, reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -34, // on EOF, reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -52, // on EOF, reduce `Relation = Identifier, ("." <Identifier>)+ => ActionFn(73);`
        0, // on EOF, error
        -42, // on EOF, reduce `Query = "SELECT", Comma<SelectItem>, QueryFrom => ActionFn(81);`
        -43, // on EOF, reduce `Query = "SELECT", Comma<SelectItem>, QueryWhere => ActionFn(82);`
        0, // on EOF, error
        0, // on EOF, error
        -65, // on EOF, reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on EOF, error
        -24, // on EOF, reduce `Comma<SelectItem> = SelectItem, (<SelectItem> ",")+ => ActionFn(71);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -6, // on EOF, reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -41, // on EOF, reduce `Query = "SELECT", Comma<SelectItem>, QueryFrom, QueryWhere => ActionFn(80);`
        -45, // on EOF, reduce `QueryFrom = "FROM", Comma<Relation> => ActionFn(4);`
        -51, // on EOF, reduce `Relation = Identifier => ActionFn(72);`
        -21, // on EOF, reduce `Comma<Relation> = Relation => ActionFn(66);`
        -33, // on EOF, reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -34, // on EOF, reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -63, // on EOF, reduce `ValueExpression = Boolean => ActionFn(19);`
        -32, // on EOF, reduce `Expression = BooleanExpression => ActionFn(15);`
        -48, // on EOF, reduce `QueryWhere = "WHERE", Expression => ActionFn(5);`
        -64, // on EOF, reduce `ValueExpression = Identifier => ActionFn(76);`
        -62, // on EOF, reduce `ValueExpression = Number => ActionFn(18);`
        -31, // on EOF, reduce `Expression = ValueExpression => ActionFn(14);`
        0, // on EOF, error
        -19, // on EOF, reduce `Boolean = "FALSE" => ActionFn(26);`
        -61, // on EOF, reduce `ValueExpression = "NULL" => ActionFn(17);`
        -18, // on EOF, reduce `Boolean = "TRUE" => ActionFn(25);`
        -36, // on EOF, reduce `Number = "r[0-9]+" => ActionFn(22);`
        -37, // on EOF, reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -38, // on EOF, reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -33, // on EOF, reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -34, // on EOF, reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        0, // on EOF, error
        -6, // on EOF, reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -54, // on EOF, reduce `SelectItem = Identifier, ".", "*" => ActionFn(74);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -16, // on EOF, reduce `(<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);`
        0, // on EOF, error
        0, // on EOF, error
        -63, // on EOF, reduce `ValueExpression = Boolean => ActionFn(19);`
        -64, // on EOF, reduce `ValueExpression = Identifier => ActionFn(76);`
        -62, // on EOF, reduce `ValueExpression = Number => ActionFn(18);`
        -20, // on EOF, reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        0, // on EOF, error
        -19, // on EOF, reduce `Boolean = "FALSE" => ActionFn(26);`
        -61, // on EOF, reduce `ValueExpression = "NULL" => ActionFn(17);`
        -18, // on EOF, reduce `Boolean = "TRUE" => ActionFn(25);`
        -36, // on EOF, reduce `Number = "r[0-9]+" => ActionFn(22);`
        -37, // on EOF, reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -38, // on EOF, reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -33, // on EOF, reduce `Identifier = r#"\"[^\"]*\""# => ActionFn(12);`
        -34, // on EOF, reduce `Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);`
        -66, // on EOF, reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -52, // on EOF, reduce `Relation = Identifier, ("." <Identifier>)+ => ActionFn(73);`
        0, // on EOF, error
        -22, // on EOF, reduce `Comma<Relation> = Relation, (<Relation> ",")+ => ActionFn(67);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -65, // on EOF, reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -55, // on EOF, reduce `SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);`
        -17, // on EOF, reduce `(<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -65, // on EOF, reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -6, // on EOF, reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -11, // on EOF, reduce `(<Relation> ",")+ = Relation, "," => ActionFn(64);`
        0, // on EOF, error
        -6, // on EOF, reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -63, // on EOF, reduce `ValueExpression = Boolean => ActionFn(19);`
        -64, // on EOF, reduce `ValueExpression = Identifier => ActionFn(76);`
        -62, // on EOF, reduce `ValueExpression = Number => ActionFn(18);`
        -20, // on EOF, reduce `BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);`
        0, // on EOF, error
        -19, // on EOF, reduce `Boolean = "FALSE" => ActionFn(26);`
        -61, // on EOF, reduce `ValueExpression = "NULL" => ActionFn(17);`
        -18, // on EOF, reduce `Boolean = "TRUE" => ActionFn(25);`
        -36, // on EOF, reduce `Number = "r[0-9]+" => ActionFn(22);`
        -37, // on EOF, reduce `Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);`
        -38, // on EOF, reduce `Number = "r\\.[0-9]+" => ActionFn(24);`
        -66, // on EOF, reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -6, // on EOF, reduce `("." <Identifier>)+ = ".", Identifier => ActionFn(60);`
        -66, // on EOF, reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -12, // on EOF, reduce `(<Relation> ",")+ = (<Relation> ",")+, Relation, "," => ActionFn(65);`
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        -65, // on EOF, reduce `ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);`
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -66, // on EOF, reduce `ValueExpression = "(", Expression, ")" => ActionFn(21);`
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        2, // on Insert, goto 1
        0, // on Number, error
        0, // on QualifiedName, error
        3, // on Query, goto 2
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        4, // on Statement, goto 3
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 1
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 2
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 3
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 4
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        9, // on Identifier, goto 8
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        10, // on Relation, goto 9
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 5
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        13, // on Boolean, goto 12
        14, // on BooleanExpression, goto 13
        0, // on Comma<Relation>, error
        15, // on Comma<SelectItem>, goto 14
        0, // on ComparisonOperator, error
        16, // on Expression, goto 15
        17, // on Identifier, goto 16
        0, // on Insert, error
        18, // on Number, goto 17
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        19, // on SelectItem, goto 18
        0, // on Statement, error
        20, // on ValueExpression, goto 19
        0, // on __Statement, error
        // State 6
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 7
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 8
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        31, // on ("." <Identifier>)+, goto 30
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 9
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 10
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 11
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 12
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 13
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 14
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        33, // on QueryFrom, goto 32
        0, // on QueryFrom?, error
        34, // on QueryWhere, goto 33
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 15
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 16
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        37, // on ("." <Identifier>)+, goto 36
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 17
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 18
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        39, // on (<SelectItem> ",")+, goto 38
        40, // on Boolean, goto 39
        41, // on BooleanExpression, goto 40
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        42, // on Expression, goto 41
        43, // on Identifier, goto 42
        0, // on Insert, error
        44, // on Number, goto 43
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        45, // on SelectItem, goto 44
        0, // on Statement, error
        46, // on ValueExpression, goto 45
        0, // on __Statement, error
        // State 19
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        57, // on ComparisonOperator, goto 56
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 20
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        64, // on Boolean, goto 63
        65, // on BooleanExpression, goto 64
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        66, // on Expression, goto 65
        67, // on Identifier, goto 66
        0, // on Insert, error
        68, // on Number, goto 67
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        69, // on ValueExpression, goto 68
        0, // on __Statement, error
        // State 21
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 22
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 23
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 24
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 25
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 26
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 27
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 28
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 29
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 30
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 31
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        80, // on Identifier, goto 79
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 32
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        81, // on QueryWhere, goto 80
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 33
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 34
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        82, // on Comma<Relation>, goto 81
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        83, // on Identifier, goto 82
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        84, // on Relation, goto 83
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 35
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        87, // on Boolean, goto 86
        88, // on BooleanExpression, goto 87
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        89, // on Expression, goto 88
        90, // on Identifier, goto 89
        0, // on Insert, error
        91, // on Number, goto 90
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        92, // on ValueExpression, goto 91
        0, // on __Statement, error
        // State 36
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 37
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        103, // on Identifier, goto 102
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 38
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        40, // on Boolean, goto 39
        41, // on BooleanExpression, goto 40
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        42, // on Expression, goto 41
        43, // on Identifier, goto 42
        0, // on Insert, error
        44, // on Number, goto 43
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        105, // on SelectItem, goto 104
        0, // on Statement, error
        46, // on ValueExpression, goto 45
        0, // on __Statement, error
        // State 39
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 40
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 41
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 42
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        106, // on ("." <Identifier>)+, goto 105
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 43
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 44
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 45
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        109, // on ComparisonOperator, goto 108
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 46
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        64, // on Boolean, goto 63
        65, // on BooleanExpression, goto 64
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        110, // on Expression, goto 109
        67, // on Identifier, goto 66
        0, // on Insert, error
        68, // on Number, goto 67
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        69, // on ValueExpression, goto 68
        0, // on __Statement, error
        // State 47
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 48
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 49
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 50
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 51
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 52
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 53
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 54
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 55
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 56
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        111, // on Boolean, goto 110
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        112, // on Identifier, goto 111
        0, // on Insert, error
        113, // on Number, goto 112
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        114, // on ValueExpression, goto 113
        0, // on __Statement, error
        // State 57
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 58
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 59
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 60
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 61
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 62
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 63
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 64
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 65
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 66
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        125, // on ("." <Identifier>)+, goto 124
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 67
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 68
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        127, // on ComparisonOperator, goto 126
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 69
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        64, // on Boolean, goto 63
        65, // on BooleanExpression, goto 64
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        128, // on Expression, goto 127
        67, // on Identifier, goto 66
        0, // on Insert, error
        68, // on Number, goto 67
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        69, // on ValueExpression, goto 68
        0, // on __Statement, error
        // State 70
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 71
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 72
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 73
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 74
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 75
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 76
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 77
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 78
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        129, // on Identifier, goto 128
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 79
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 80
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 81
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 82
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        130, // on ("." <Identifier>)+, goto 129
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 83
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        132, // on (<Relation> ",")+, goto 131
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        133, // on Identifier, goto 132
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        134, // on Relation, goto 133
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 84
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 85
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 86
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 87
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 88
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 89
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        137, // on ("." <Identifier>)+, goto 136
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 90
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 91
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        139, // on ComparisonOperator, goto 138
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 92
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        64, // on Boolean, goto 63
        65, // on BooleanExpression, goto 64
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        140, // on Expression, goto 139
        67, // on Identifier, goto 66
        0, // on Insert, error
        68, // on Number, goto 67
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        69, // on ValueExpression, goto 68
        0, // on __Statement, error
        // State 93
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 94
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 95
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 96
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 97
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 98
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 99
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 100
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 101
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        141, // on Identifier, goto 140
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 102
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 103
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 104
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 105
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 106
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        145, // on Identifier, goto 144
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 107
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 108
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        147, // on Boolean, goto 146
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        148, // on Identifier, goto 147
        0, // on Insert, error
        149, // on Number, goto 148
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        150, // on ValueExpression, goto 149
        0, // on __Statement, error
        // State 109
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 110
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 111
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        159, // on ("." <Identifier>)+, goto 158
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 112
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 113
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 114
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        64, // on Boolean, goto 63
        65, // on BooleanExpression, goto 64
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        161, // on Expression, goto 160
        67, // on Identifier, goto 66
        0, // on Insert, error
        68, // on Number, goto 67
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        69, // on ValueExpression, goto 68
        0, // on __Statement, error
        // State 115
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 116
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 117
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 118
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 119
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 120
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 121
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 122
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 123
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 124
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 125
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        163, // on Identifier, goto 162
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 126
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        164, // on Boolean, goto 163
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        165, // on Identifier, goto 164
        0, // on Insert, error
        166, // on Number, goto 165
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        167, // on ValueExpression, goto 166
        0, // on __Statement, error
        // State 127
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 128
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 129
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 130
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        179, // on Identifier, goto 178
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 131
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        133, // on Identifier, goto 132
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        180, // on Relation, goto 179
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 132
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        181, // on ("." <Identifier>)+, goto 180
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 133
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 134
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 135
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 136
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 137
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        185, // on Identifier, goto 184
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 138
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        186, // on Boolean, goto 185
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        187, // on Identifier, goto 186
        0, // on Insert, error
        188, // on Number, goto 187
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        189, // on ValueExpression, goto 188
        0, // on __Statement, error
        // State 139
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 140
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 141
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 142
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 143
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        198, // on Identifier, goto 197
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 144
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 145
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 146
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 147
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        200, // on ("." <Identifier>)+, goto 199
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 148
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 149
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 150
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        64, // on Boolean, goto 63
        65, // on BooleanExpression, goto 64
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        201, // on Expression, goto 200
        67, // on Identifier, goto 66
        0, // on Insert, error
        68, // on Number, goto 67
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        69, // on ValueExpression, goto 68
        0, // on __Statement, error
        // State 151
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 152
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 153
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 154
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 155
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 156
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 157
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 158
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 159
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        203, // on Identifier, goto 202
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 160
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 161
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        205, // on Identifier, goto 204
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 162
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 163
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 164
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        206, // on ("." <Identifier>)+, goto 205
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 165
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 166
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 167
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        64, // on Boolean, goto 63
        65, // on BooleanExpression, goto 64
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        208, // on Expression, goto 207
        67, // on Identifier, goto 66
        0, // on Insert, error
        68, // on Number, goto 67
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        69, // on ValueExpression, goto 68
        0, // on __Statement, error
        // State 168
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 169
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 170
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 171
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 172
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 173
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 174
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 175
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 176
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 177
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        209, // on Identifier, goto 208
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 178
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 179
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 180
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 181
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        212, // on Identifier, goto 211
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 182
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 183
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        213, // on Identifier, goto 212
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 184
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 185
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 186
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        214, // on ("." <Identifier>)+, goto 213
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 187
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 188
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 189
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        64, // on Boolean, goto 63
        65, // on BooleanExpression, goto 64
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        215, // on Expression, goto 214
        67, // on Identifier, goto 66
        0, // on Insert, error
        68, // on Number, goto 67
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        69, // on ValueExpression, goto 68
        0, // on __Statement, error
        // State 190
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 191
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 192
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 193
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 194
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 195
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 196
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 197
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 198
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 199
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 200
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 201
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        217, // on Identifier, goto 216
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 202
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 203
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 204
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 205
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 206
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        219, // on Identifier, goto 218
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 207
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 208
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 209
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 210
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        221, // on Identifier, goto 220
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 211
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 212
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 213
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 214
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 215
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 216
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 217
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        223, // on Identifier, goto 222
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 218
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 219
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 220
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 221
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
        // State 222
        0, // on ";"?, error
        0, // on ("." <Identifier>), error
        0, // on ("." <Identifier>)*, error
        0, // on ("." <Identifier>)+, error
        0, // on (<Relation> ","), error
        0, // on (<Relation> ",")*, error
        0, // on (<Relation> ",")+, error
        0, // on (<SelectItem> ","), error
        0, // on (<SelectItem> ",")*, error
        0, // on (<SelectItem> ",")+, error
        0, // on Boolean, error
        0, // on BooleanExpression, error
        0, // on Comma<Relation>, error
        0, // on Comma<SelectItem>, error
        0, // on ComparisonOperator, error
        0, // on Expression, error
        0, // on Identifier, error
        0, // on Insert, error
        0, // on Number, error
        0, // on QualifiedName, error
        0, // on Query, error
        0, // on QueryFrom, error
        0, // on QueryFrom?, error
        0, // on QueryWhere, error
        0, // on QueryWhere?, error
        0, // on Relation, error
        0, // on SelectItem, error
        0, // on Statement, error
        0, // on ValueExpression, error
        0, // on __Statement, error
    ];
    pub fn parse_Statement<
        'input,
    >(
        input: &'input str,
    ) -> Result<Box<Statement>, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                (_, (8, _), _) if true => 8,
                (_, (9, _), _) if true => 9,
                (_, (10, _), _) if true => 10,
                (_, (11, _), _) if true => 11,
                (_, (12, _), _) if true => 12,
                (_, (13, _), _) if true => 13,
                (_, (14, _), _) if true => 14,
                (_, (15, _), _) if true => 15,
                (_, (16, _), _) if true => 16,
                (_, (17, _), _) if true => 17,
                (_, (18, _), _) if true => 18,
                (_, (19, _), _) if true => 19,
                (_, (20, _), _) if true => 20,
                (_, (21, _), _) if true => 21,
                (_, (22, _), _) if true => 22,
                (_, (23, _), _) if true => 23,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 24 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_21_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2c_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2e_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_3b_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_3c_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_3c_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_3e_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3e_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22FALSE_22(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22FROM_22(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22INSERT_20INTO_22(__tok0),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22NULL_22(__tok0),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22SELECT_22(__tok0),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22TRUE_22(__tok0),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22WHERE_22(__tok0),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            (19, __tok0) => __Symbol::Term_22r_5b0_2d9_5d_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            (20, __tok0) => __Symbol::Term_22r_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            (21, __tok0) => __Symbol::Term_22r_5c_5c_2e_5b0_2d9_5d_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            (22, __tok0) => __Symbol::Termr_23_22_5c_22_5b_5e_5c_22_5d_2a_5c_22_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            (23, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ___5d_5ba_2dzA_2dZ0_2d9_40_3a___5d_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Box<Statement>,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ";"? = ";" => ActionFn(42);
                let __sym0 = __pop_Term_22_3b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22_3b_22_3f(__nt), __end));
                0
            }
            2 => {
                // ";"? =  => ActionFn(43);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action43::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22_3b_22_3f(__nt), __end));
                0
            }
            3 => {
                // ("." <Identifier>) = ".", Identifier => ActionFn(35);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22_2e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2e_22_20_3cIdentifier_3e_29(__nt), __end));
                1
            }
            4 => {
                // ("." <Identifier>)* =  => ActionFn(33);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action33::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22_2e_22_20_3cIdentifier_3e_29_2a(__nt), __end));
                2
            }
            5 => {
                // ("." <Identifier>)* = ("." <Identifier>)+ => ActionFn(34);
                let __sym0 = __pop_Nt_28_22_2e_22_20_3cIdentifier_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_22_2e_22_20_3cIdentifier_3e_29_2a(__nt), __end));
                2
            }
            6 => {
                // ("." <Identifier>)+ = ".", Identifier => ActionFn(60);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22_2e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action60::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2e_22_20_3cIdentifier_3e_29_2b(__nt), __end));
                3
            }
            7 => {
                // ("." <Identifier>)+ = ("." <Identifier>)+, ".", Identifier => ActionFn(61);
                let __sym2 = __pop_NtIdentifier(__symbols);
                let __sym1 = __pop_Term_22_2e_22(__symbols);
                let __sym0 = __pop_Nt_28_22_2e_22_20_3cIdentifier_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action61::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22_2e_22_20_3cIdentifier_3e_29_2b(__nt), __end));
                3
            }
            8 => {
                // (<Relation> ",") = Relation, "," => ActionFn(49);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtRelation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action49::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cRelation_3e_20_22_2c_22_29(__nt), __end));
                4
            }
            9 => {
                // (<Relation> ",")* =  => ActionFn(47);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action47::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cRelation_3e_20_22_2c_22_29_2a(__nt), __end));
                5
            }
            10 => {
                // (<Relation> ",")* = (<Relation> ",")+ => ActionFn(48);
                let __sym0 = __pop_Nt_28_3cRelation_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action48::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cRelation_3e_20_22_2c_22_29_2a(__nt), __end));
                5
            }
            11 => {
                // (<Relation> ",")+ = Relation, "," => ActionFn(64);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtRelation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action64::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cRelation_3e_20_22_2c_22_29_2b(__nt), __end));
                6
            }
            12 => {
                // (<Relation> ",")+ = (<Relation> ",")+, Relation, "," => ActionFn(65);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtRelation(__symbols);
                let __sym0 = __pop_Nt_28_3cRelation_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action65::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cRelation_3e_20_22_2c_22_29_2b(__nt), __end));
                6
            }
            13 => {
                // (<SelectItem> ",") = SelectItem, "," => ActionFn(46);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtSelectItem(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cSelectItem_3e_20_22_2c_22_29(__nt), __end));
                7
            }
            14 => {
                // (<SelectItem> ",")* =  => ActionFn(44);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action44::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cSelectItem_3e_20_22_2c_22_29_2a(__nt), __end));
                8
            }
            15 => {
                // (<SelectItem> ",")* = (<SelectItem> ",")+ => ActionFn(45);
                let __sym0 = __pop_Nt_28_3cSelectItem_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cSelectItem_3e_20_22_2c_22_29_2a(__nt), __end));
                8
            }
            16 => {
                // (<SelectItem> ",")+ = SelectItem, "," => ActionFn(68);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtSelectItem(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action68::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cSelectItem_3e_20_22_2c_22_29_2b(__nt), __end));
                9
            }
            17 => {
                // (<SelectItem> ",")+ = (<SelectItem> ",")+, SelectItem, "," => ActionFn(69);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtSelectItem(__symbols);
                let __sym0 = __pop_Nt_28_3cSelectItem_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action69::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cSelectItem_3e_20_22_2c_22_29_2b(__nt), __end));
                9
            }
            18 => {
                // Boolean = "TRUE" => ActionFn(25);
                let __sym0 = __pop_Term_22TRUE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBoolean(__nt), __end));
                10
            }
            19 => {
                // Boolean = "FALSE" => ActionFn(26);
                let __sym0 = __pop_Term_22FALSE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBoolean(__nt), __end));
                10
            }
            20 => {
                // BooleanExpression = ValueExpression, ComparisonOperator, ValueExpression => ActionFn(16);
                let __sym2 = __pop_NtValueExpression(__symbols);
                let __sym1 = __pop_NtComparisonOperator(__symbols);
                let __sym0 = __pop_NtValueExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBooleanExpression(__nt), __end));
                11
            }
            21 => {
                // Comma<Relation> = Relation => ActionFn(66);
                let __sym0 = __pop_NtRelation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action66::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cRelation_3e(__nt), __end));
                12
            }
            22 => {
                // Comma<Relation> = Relation, (<Relation> ",")+ => ActionFn(67);
                let __sym1 = __pop_Nt_28_3cRelation_3e_20_22_2c_22_29_2b(__symbols);
                let __sym0 = __pop_NtRelation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action67::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cRelation_3e(__nt), __end));
                12
            }
            23 => {
                // Comma<SelectItem> = SelectItem => ActionFn(70);
                let __sym0 = __pop_NtSelectItem(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action70::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cSelectItem_3e(__nt), __end));
                13
            }
            24 => {
                // Comma<SelectItem> = SelectItem, (<SelectItem> ",")+ => ActionFn(71);
                let __sym1 = __pop_Nt_28_3cSelectItem_3e_20_22_2c_22_29_2b(__symbols);
                let __sym0 = __pop_NtSelectItem(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action71::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cSelectItem_3e(__nt), __end));
                13
            }
            25 => {
                // ComparisonOperator = "=" => ActionFn(27);
                let __sym0 = __pop_Term_22_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                14
            }
            26 => {
                // ComparisonOperator = "!=" => ActionFn(28);
                let __sym0 = __pop_Term_22_21_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                14
            }
            27 => {
                // ComparisonOperator = ">" => ActionFn(29);
                let __sym0 = __pop_Term_22_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                14
            }
            28 => {
                // ComparisonOperator = ">=" => ActionFn(30);
                let __sym0 = __pop_Term_22_3e_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                14
            }
            29 => {
                // ComparisonOperator = "<" => ActionFn(31);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                14
            }
            30 => {
                // ComparisonOperator = "<=" => ActionFn(32);
                let __sym0 = __pop_Term_22_3c_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                14
            }
            31 => {
                // Expression = ValueExpression => ActionFn(14);
                let __sym0 = __pop_NtValueExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                15
            }
            32 => {
                // Expression = BooleanExpression => ActionFn(15);
                let __sym0 = __pop_NtBooleanExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                15
            }
            33 => {
                // Identifier = r#"\"[^\"]*\""# => ActionFn(12);
                let __sym0 = __pop_Termr_23_22_5c_22_5b_5e_5c_22_5d_2a_5c_22_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdentifier(__nt), __end));
                16
            }
            34 => {
                // Identifier = r#"[a-zA-Z_][a-zA-Z0-9@:_]*"# => ActionFn(13);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ___5d_5ba_2dzA_2dZ0_2d9_40_3a___5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdentifier(__nt), __end));
                16
            }
            35 => {
                // Insert = "INSERT INTO", Relation => ActionFn(6);
                let __sym1 = __pop_NtRelation(__symbols);
                let __sym0 = __pop_Term_22INSERT_20INTO_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtInsert(__nt), __end));
                17
            }
            36 => {
                // Number = "r[0-9]+" => ActionFn(22);
                let __sym0 = __pop_Term_22r_5b0_2d9_5d_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumber(__nt), __end));
                18
            }
            37 => {
                // Number = "r[0-9]+\\.[0-9]*" => ActionFn(23);
                let __sym0 = __pop_Term_22r_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumber(__nt), __end));
                18
            }
            38 => {
                // Number = "r\\.[0-9]+" => ActionFn(24);
                let __sym0 = __pop_Term_22r_5c_5c_2e_5b0_2d9_5d_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumber(__nt), __end));
                18
            }
            39 => {
                // QualifiedName = Identifier => ActionFn(62);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action62::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtQualifiedName(__nt), __end));
                19
            }
            40 => {
                // QualifiedName = Identifier, ("." <Identifier>)+ => ActionFn(63);
                let __sym1 = __pop_Nt_28_22_2e_22_20_3cIdentifier_3e_29_2b(__symbols);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action63::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtQualifiedName(__nt), __end));
                19
            }
            41 => {
                // Query = "SELECT", Comma<SelectItem>, QueryFrom, QueryWhere => ActionFn(80);
                let __sym3 = __pop_NtQueryWhere(__symbols);
                let __sym2 = __pop_NtQueryFrom(__symbols);
                let __sym1 = __pop_NtComma_3cSelectItem_3e(__symbols);
                let __sym0 = __pop_Term_22SELECT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action80::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtQuery(__nt), __end));
                20
            }
            42 => {
                // Query = "SELECT", Comma<SelectItem>, QueryFrom => ActionFn(81);
                let __sym2 = __pop_NtQueryFrom(__symbols);
                let __sym1 = __pop_NtComma_3cSelectItem_3e(__symbols);
                let __sym0 = __pop_Term_22SELECT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action81::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtQuery(__nt), __end));
                20
            }
            43 => {
                // Query = "SELECT", Comma<SelectItem>, QueryWhere => ActionFn(82);
                let __sym2 = __pop_NtQueryWhere(__symbols);
                let __sym1 = __pop_NtComma_3cSelectItem_3e(__symbols);
                let __sym0 = __pop_Term_22SELECT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action82::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtQuery(__nt), __end));
                20
            }
            44 => {
                // Query = "SELECT", Comma<SelectItem> => ActionFn(83);
                let __sym1 = __pop_NtComma_3cSelectItem_3e(__symbols);
                let __sym0 = __pop_Term_22SELECT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action83::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtQuery(__nt), __end));
                20
            }
            45 => {
                // QueryFrom = "FROM", Comma<Relation> => ActionFn(4);
                let __sym1 = __pop_NtComma_3cRelation_3e(__symbols);
                let __sym0 = __pop_Term_22FROM_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action4::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtQueryFrom(__nt), __end));
                21
            }
            46 => {
                // QueryFrom? = QueryFrom => ActionFn(39);
                let __sym0 = __pop_NtQueryFrom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtQueryFrom_3f(__nt), __end));
                22
            }
            47 => {
                // QueryFrom? =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtQueryFrom_3f(__nt), __end));
                22
            }
            48 => {
                // QueryWhere = "WHERE", Expression => ActionFn(5);
                let __sym1 = __pop_NtExpression(__symbols);
                let __sym0 = __pop_Term_22WHERE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtQueryWhere(__nt), __end));
                23
            }
            49 => {
                // QueryWhere? = QueryWhere => ActionFn(37);
                let __sym0 = __pop_NtQueryWhere(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtQueryWhere_3f(__nt), __end));
                24
            }
            50 => {
                // QueryWhere? =  => ActionFn(38);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action38::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtQueryWhere_3f(__nt), __end));
                24
            }
            51 => {
                // Relation = Identifier => ActionFn(72);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action72::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtRelation(__nt), __end));
                25
            }
            52 => {
                // Relation = Identifier, ("." <Identifier>)+ => ActionFn(73);
                let __sym1 = __pop_Nt_28_22_2e_22_20_3cIdentifier_3e_29_2b(__symbols);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action73::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRelation(__nt), __end));
                25
            }
            53 => {
                // SelectItem = Expression => ActionFn(7);
                let __sym0 = __pop_NtExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSelectItem(__nt), __end));
                26
            }
            54 => {
                // SelectItem = Identifier, ".", "*" => ActionFn(74);
                let __sym2 = __pop_Term_22_2a_22(__symbols);
                let __sym1 = __pop_Term_22_2e_22(__symbols);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action74::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSelectItem(__nt), __end));
                26
            }
            55 => {
                // SelectItem = Identifier, ("." <Identifier>)+, ".", "*" => ActionFn(75);
                let __sym3 = __pop_Term_22_2a_22(__symbols);
                let __sym2 = __pop_Term_22_2e_22(__symbols);
                let __sym1 = __pop_Nt_28_22_2e_22_20_3cIdentifier_3e_29_2b(__symbols);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action75::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtSelectItem(__nt), __end));
                26
            }
            56 => {
                // SelectItem = "*" => ActionFn(9);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSelectItem(__nt), __end));
                26
            }
            57 => {
                // Statement = Query, ";" => ActionFn(56);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtQuery(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action56::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                27
            }
            58 => {
                // Statement = Query => ActionFn(57);
                let __sym0 = __pop_NtQuery(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                27
            }
            59 => {
                // Statement = Insert, ";" => ActionFn(58);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtInsert(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                27
            }
            60 => {
                // Statement = Insert => ActionFn(59);
                let __sym0 = __pop_NtInsert(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                27
            }
            61 => {
                // ValueExpression = "NULL" => ActionFn(17);
                let __sym0 = __pop_Term_22NULL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValueExpression(__nt), __end));
                28
            }
            62 => {
                // ValueExpression = Number => ActionFn(18);
                let __sym0 = __pop_NtNumber(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValueExpression(__nt), __end));
                28
            }
            63 => {
                // ValueExpression = Boolean => ActionFn(19);
                let __sym0 = __pop_NtBoolean(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValueExpression(__nt), __end));
                28
            }
            64 => {
                // ValueExpression = Identifier => ActionFn(76);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action76::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValueExpression(__nt), __end));
                28
            }
            65 => {
                // ValueExpression = Identifier, ("." <Identifier>)+ => ActionFn(77);
                let __sym1 = __pop_Nt_28_22_2e_22_20_3cIdentifier_3e_29_2b(__symbols);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action77::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtValueExpression(__nt), __end));
                28
            }
            66 => {
                // ValueExpression = "(", Expression, ")" => ActionFn(21);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtValueExpression(__nt), __end));
                28
            }
            67 => {
                // __Statement = Statement => ActionFn(0);
                let __sym0 = __pop_NtStatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 30 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_21_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_21_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22FALSE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22FALSE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22FROM_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22FROM_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22INSERT_20INTO_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22INSERT_20INTO_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22NULL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22NULL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22SELECT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22SELECT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22TRUE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22TRUE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22WHERE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22WHERE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22r_5b0_2d9_5d_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22r_5b0_2d9_5d_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22r_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22r_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22r_5c_5c_2e_5b0_2d9_5d_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22r_5c_5c_2e_5b0_2d9_5d_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_22_5b_5e_5c_22_5d_2a_5c_22_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_22_5b_5e_5c_22_5d_2a_5c_22_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ___5d_5ba_2dzA_2dZ0_2d9_40_3a___5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ___5d_5ba_2dzA_2dZ0_2d9_40_3a___5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22_3b_22_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22_3b_22_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2e_22_20_3cIdentifier_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2e_22_20_3cIdentifier_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2e_22_20_3cIdentifier_3e_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2e_22_20_3cIdentifier_3e_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2e_22_20_3cIdentifier_3e_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2e_22_20_3cIdentifier_3e_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cRelation_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Relation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cRelation_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cRelation_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Relation>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cRelation_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cRelation_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Relation>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cRelation_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSelectItem_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SelectItem, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSelectItem_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSelectItem_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<SelectItem>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSelectItem_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSelectItem_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<SelectItem>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSelectItem_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBoolean<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBoolean(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBooleanExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBooleanExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cRelation_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Relation>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cRelation_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cSelectItem_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<SelectItem>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cSelectItem_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComparisonOperator<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ComparisonOperator, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComparisonOperator(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdentifier<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdentifier(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtInsert<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, InsertStatement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInsert(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumber<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumber(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtQualifiedName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, QualifiedName, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtQualifiedName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtQuery<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SelectStatement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtQuery(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtQueryFrom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Relation>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtQueryFrom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtQueryFrom_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Vec<Relation>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtQueryFrom_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtQueryWhere<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtQueryWhere(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtQueryWhere_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Box<Expression>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtQueryWhere_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRelation<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Relation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRelation(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSelectItem<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SelectItem, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSelectItem(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtStatement<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtValueExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtValueExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Statement<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Statement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Statement::parse_Statement;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        33 => /* '!' */ {
                            __current_state = 1;
                            continue;
                        }
                        34 => /* '\"' */ {
                            __current_state = 2;
                            continue;
                        }
                        40 => /* '(' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        42 => /* '*' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        44 => /* ',' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        46 => /* '.' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        59 => /* ';' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        60 => /* '<' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        61 => /* '=' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        62 => /* '>' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        65 ... 69 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        70 => /* 'F' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        71 ... 72 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        73 => /* 'I' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        74 ... 77 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        78 => /* 'N' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        79 ... 82 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        83 => /* 'S' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        84 => /* 'T' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        85 ... 86 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        87 => /* 'W' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        88 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        61 => /* '=' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        0 ... 33 => {
                            __current_state = 22;
                            continue;
                        }
                        34 => /* '\"' */ {
                            __current_match = Some((22, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        35 ... 1114111 => {
                            __current_state = 22;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        61 => /* '=' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        61 => /* '=' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 => /* '@' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        65 => /* 'A' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        66 ... 81 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        82 => /* 'R' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        83 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 77 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        78 => /* 'N' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        79 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 84 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        85 => /* 'U' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        86 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 68 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        69 => /* 'E' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 31;
                            continue;
                        }
                        70 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 81 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        82 => /* 'R' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        83 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 71 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        72 => /* 'H' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 33;
                            continue;
                        }
                        73 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        91 => /* '[' */ {
                            __current_state = 34;
                            continue;
                        }
                        92 => /* '\\' */ {
                            __current_state = 35;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        0 ... 33 => {
                            __current_state = 22;
                            continue;
                        }
                        34 => /* '\"' */ {
                            __current_match = Some((22, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        35 ... 1114111 => {
                            __current_state = 22;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 75 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        76 => /* 'L' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 36;
                            continue;
                        }
                        77 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 78 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        79 => /* 'O' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 37;
                            continue;
                        }
                        80 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 82 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        83 => /* 'S' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 38;
                            continue;
                        }
                        84 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 75 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        76 => /* 'L' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 39;
                            continue;
                        }
                        77 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 75 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        76 => /* 'L' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        77 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 84 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        85 => /* 'U' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 41;
                            continue;
                        }
                        86 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 68 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        69 => /* 'E' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 42;
                            continue;
                        }
                        70 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 => /* '0' */ {
                            __current_state = 43;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        46 => /* '.' */ {
                            __current_state = 44;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 82 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        83 => /* 'S' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 45;
                            continue;
                        }
                        84 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                37 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 76 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        77 => /* 'M' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        78 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                38 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 68 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        69 => /* 'E' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 47;
                            continue;
                        }
                        70 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                39 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 75 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        76 => /* 'L' */ {
                            __current_match = Some((15, __index + 1));
                            __current_state = 48;
                            continue;
                        }
                        77 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                40 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 68 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        69 => /* 'E' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 49;
                            continue;
                        }
                        70 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                41 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 68 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        69 => /* 'E' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 50;
                            continue;
                        }
                        70 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                42 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 81 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        82 => /* 'R' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 51;
                            continue;
                        }
                        83 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                43 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        45 => /* '-' */ {
                            __current_state = 52;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                44 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        91 => /* '[' */ {
                            __current_state = 53;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                45 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 68 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        69 => /* 'E' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 54;
                            continue;
                        }
                        70 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                46 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                47 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 81 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        82 => /* 'R' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 55;
                            continue;
                        }
                        83 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                48 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                49 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 66 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        67 => /* 'C' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 56;
                            continue;
                        }
                        68 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                50 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                51 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 68 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        69 => /* 'E' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 57;
                            continue;
                        }
                        70 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                52 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        57 => /* '9' */ {
                            __current_state = 58;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                53 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 => /* '0' */ {
                            __current_state = 59;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                54 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                55 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 83 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        84 => /* 'T' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 60;
                            continue;
                        }
                        85 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                56 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 83 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        84 => /* 'T' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 61;
                            continue;
                        }
                        85 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                57 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                58 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        93 => /* ']' */ {
                            __current_state = 62;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                59 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        45 => /* '-' */ {
                            __current_state = 63;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                60 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_state = 64;
                            continue;
                        }
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                61 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 58 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        64 ... 90 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((23, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                62 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        43 => /* '+' */ {
                            __current_match = Some((19, __index + 1));
                            __current_state = 65;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                63 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        57 => /* '9' */ {
                            __current_state = 66;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                64 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        73 => /* 'I' */ {
                            __current_state = 67;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                65 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        92 => /* '\\' */ {
                            __current_state = 68;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                66 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        93 => /* ']' */ {
                            __current_state = 69;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                67 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_state = 70;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                68 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        46 => /* '.' */ {
                            __current_state = 71;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                69 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        43 => /* '+' */ {
                            __current_match = Some((21, __index + 1));
                            __current_state = 72;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                70 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        84 => /* 'T' */ {
                            __current_state = 73;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                71 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        91 => /* '[' */ {
                            __current_state = 74;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                72 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                73 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        79 => /* 'O' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 75;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                74 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 => /* '0' */ {
                            __current_state = 76;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                75 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                76 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        45 => /* '-' */ {
                            __current_state = 77;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                77 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        57 => /* '9' */ {
                            __current_state = 78;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                78 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        93 => /* ']' */ {
                            __current_state = 79;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                79 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        42 => /* '*' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 80;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                80 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Statement>, usize),
) -> Box<Statement>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SelectStatement, usize),
    (_, _, _): (usize, ::std::option::Option<&'input str>, usize),
) -> Box<Statement>
{
    Box::new(Statement::Select(__0))
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, InsertStatement, usize),
    (_, _, _): (usize, ::std::option::Option<&'input str>, usize),
) -> Box<Statement>
{
    Box::new(Statement::Insert(__0))
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, selection, _): (usize, Vec<SelectItem>, usize),
    (_, from, _): (usize, ::std::option::Option<Vec<Relation>>, usize),
    (_, condition, _): (usize, ::std::option::Option<Box<Expression>>, usize),
) -> SelectStatement
{
    {
      let from: Vec<Relation> = match from {
          None => Vec::new(),
          Some(v) => v
      };
      SelectStatement { selections: selection, from_relation: from, where_condition: condition }
  }
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<Relation>, usize),
) -> Vec<Relation>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Expression, usize),
) -> Box<Expression>
{
    Box::new(__0)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Relation, usize),
) -> InsertStatement
{
    InsertStatement { into_relation: __0 }
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> SelectItem
{
    SelectItem::Expression(__0)
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, QualifiedName, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> SelectItem
{
    SelectItem::SelectAll(Some(__0))
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> SelectItem
{
    SelectItem::SelectAll(None)
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, QualifiedName, usize),
) -> Relation
{
    Relation::Named(__0)
    // "(" Query ")" => ,
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, e, _): (usize, Identifier, usize),
    (_, v, _): (usize, ::std::vec::Vec<Identifier>, usize),
) -> QualifiedName
{
    {
        let mut v = v;
        v.insert(0, e);
        v
    }
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Identifier
{
    Identifier::from(__0)
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Identifier
{
    Identifier::from(__0)
    // <a:Letter> <b:IdentChar*> => Identifier::from(String::from(a) + b)
    // <a:r"_"> <b:IdentChar+> => Identifier::from(String::from(a) + b)
    // <a:Digit> <b:IdentChar+> => Identifier::from(String::from(a) + b)
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Expression, usize),
    (_, operator, _): (usize, ComparisonOperator, usize),
    (_, right, _): (usize, Expression, usize),
) -> Expression
{
    Expression::BooleanExpression {left: Box::new(left), right: Box::new(right), operator: operator}
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::NullLiteral
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::NumberLiteral(String::from(__0))
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::BooleanLiteral(String::from(__0))
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, QualifiedName, usize),
) -> Expression
{
    Expression::QualifiedNameReference(__0)
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Expression, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::ParenthesizedExpression(Box::new(__0))
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ComparisonOperator
{
    ComparisonOperator::Equals
}

#[allow(unused_variables)]
pub fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ComparisonOperator
{
    ComparisonOperator::NotEquals
}

#[allow(unused_variables)]
pub fn __action29<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ComparisonOperator
{
    ComparisonOperator::GreaterThan
}

#[allow(unused_variables)]
pub fn __action30<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ComparisonOperator
{
    ComparisonOperator::GreaterThanEquals
}

#[allow(unused_variables)]
pub fn __action31<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ComparisonOperator
{
    ComparisonOperator::LessThan
}

#[allow(unused_variables)]
pub fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ComparisonOperator
{
    ComparisonOperator::LessThanEquals
}

#[allow(unused_variables)]
pub fn __action33<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Identifier>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action34<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Identifier>, usize),
) -> ::std::vec::Vec<Identifier>
{
    v
}

#[allow(unused_variables)]
pub fn __action35<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Identifier, usize),
) -> Identifier
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action36<
    'input,
>(
    input: &'input str,
    (_, e, _): (usize, Relation, usize),
    (_, v, _): (usize, ::std::vec::Vec<Relation>, usize),
) -> Vec<Relation>
{
    {
        let mut v = v;
        v.insert(0, e);
        v
    }
}

#[allow(unused_variables)]
pub fn __action37<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expression>, usize),
) -> ::std::option::Option<Box<Expression>>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action38<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Box<Expression>>
{
    None
}

#[allow(unused_variables)]
pub fn __action39<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Relation>, usize),
) -> ::std::option::Option<Vec<Relation>>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action40<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Vec<Relation>>
{
    None
}

#[allow(unused_variables)]
pub fn __action41<
    'input,
>(
    input: &'input str,
    (_, e, _): (usize, SelectItem, usize),
    (_, v, _): (usize, ::std::vec::Vec<SelectItem>, usize),
) -> Vec<SelectItem>
{
    {
        let mut v = v;
        v.insert(0, e);
        v
    }
}

#[allow(unused_variables)]
pub fn __action42<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::option::Option<&'input str>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action43<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<&'input str>
{
    None
}

#[allow(unused_variables)]
pub fn __action44<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<SelectItem>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action45<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<SelectItem>, usize),
) -> ::std::vec::Vec<SelectItem>
{
    v
}

#[allow(unused_variables)]
pub fn __action46<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SelectItem, usize),
    (_, _, _): (usize, &'input str, usize),
) -> SelectItem
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action47<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Relation>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action48<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Relation>, usize),
) -> ::std::vec::Vec<Relation>
{
    v
}

#[allow(unused_variables)]
pub fn __action49<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Relation, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Relation
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action50<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Identifier, usize),
) -> ::std::vec::Vec<Identifier>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action51<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Identifier>, usize),
    (_, e, _): (usize, Identifier, usize),
) -> ::std::vec::Vec<Identifier>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action52<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Relation, usize),
) -> ::std::vec::Vec<Relation>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action53<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Relation>, usize),
    (_, e, _): (usize, Relation, usize),
) -> ::std::vec::Vec<Relation>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action54<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SelectItem, usize),
) -> ::std::vec::Vec<SelectItem>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action55<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<SelectItem>, usize),
    (_, e, _): (usize, SelectItem, usize),
) -> ::std::vec::Vec<SelectItem>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action56<
    'input,
>(
    input: &'input str,
    __0: (usize, SelectStatement, usize),
    __1: (usize, &'input str, usize),
) -> Box<Statement>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action42(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action57<
    'input,
>(
    input: &'input str,
    __0: (usize, SelectStatement, usize),
) -> Box<Statement>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action58<
    'input,
>(
    input: &'input str,
    __0: (usize, InsertStatement, usize),
    __1: (usize, &'input str, usize),
) -> Box<Statement>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action42(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action59<
    'input,
>(
    input: &'input str,
    __0: (usize, InsertStatement, usize),
) -> Box<Statement>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action60<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Identifier, usize),
) -> ::std::vec::Vec<Identifier>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action35(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action61<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Identifier>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Identifier, usize),
) -> ::std::vec::Vec<Identifier>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action35(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action62<
    'input,
>(
    input: &'input str,
    __0: (usize, Identifier, usize),
) -> QualifiedName
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action33(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action63<
    'input,
>(
    input: &'input str,
    __0: (usize, Identifier, usize),
    __1: (usize, ::std::vec::Vec<Identifier>, usize),
) -> QualifiedName
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action34(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action64<
    'input,
>(
    input: &'input str,
    __0: (usize, Relation, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<Relation>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action49(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action65<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Relation>, usize),
    __1: (usize, Relation, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<Relation>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action49(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action66<
    'input,
>(
    input: &'input str,
    __0: (usize, Relation, usize),
) -> Vec<Relation>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action67<
    'input,
>(
    input: &'input str,
    __0: (usize, Relation, usize),
    __1: (usize, ::std::vec::Vec<Relation>, usize),
) -> Vec<Relation>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action48(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action68<
    'input,
>(
    input: &'input str,
    __0: (usize, SelectItem, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<SelectItem>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action46(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action69<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<SelectItem>, usize),
    __1: (usize, SelectItem, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<SelectItem>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action46(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action70<
    'input,
>(
    input: &'input str,
    __0: (usize, SelectItem, usize),
) -> Vec<SelectItem>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action71<
    'input,
>(
    input: &'input str,
    __0: (usize, SelectItem, usize),
    __1: (usize, ::std::vec::Vec<SelectItem>, usize),
) -> Vec<SelectItem>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action45(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action72<
    'input,
>(
    input: &'input str,
    __0: (usize, Identifier, usize),
) -> Relation
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action62(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action73<
    'input,
>(
    input: &'input str,
    __0: (usize, Identifier, usize),
    __1: (usize, ::std::vec::Vec<Identifier>, usize),
) -> Relation
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action63(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action74<
    'input,
>(
    input: &'input str,
    __0: (usize, Identifier, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
) -> SelectItem
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action62(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action75<
    'input,
>(
    input: &'input str,
    __0: (usize, Identifier, usize),
    __1: (usize, ::std::vec::Vec<Identifier>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
) -> SelectItem
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action63(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        __temp0,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action76<
    'input,
>(
    input: &'input str,
    __0: (usize, Identifier, usize),
) -> Expression
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action62(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action77<
    'input,
>(
    input: &'input str,
    __0: (usize, Identifier, usize),
    __1: (usize, ::std::vec::Vec<Identifier>, usize),
) -> Expression
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action63(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action78<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Vec<SelectItem>, usize),
    __2: (usize, Vec<Relation>, usize),
    __3: (usize, ::std::option::Option<Box<Expression>>, usize),
) -> SelectStatement
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action39(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action79<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Vec<SelectItem>, usize),
    __2: (usize, ::std::option::Option<Box<Expression>>, usize),
) -> SelectStatement
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action40(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action80<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Vec<SelectItem>, usize),
    __2: (usize, Vec<Relation>, usize),
    __3: (usize, Box<Expression>, usize),
) -> SelectStatement
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action37(
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action81<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Vec<SelectItem>, usize),
    __2: (usize, Vec<Relation>, usize),
) -> SelectStatement
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action38(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action82<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Vec<SelectItem>, usize),
    __2: (usize, Box<Expression>, usize),
) -> SelectStatement
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action37(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action83<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Vec<SelectItem>, usize),
) -> SelectStatement
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        input,
        __0,
        __1,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
