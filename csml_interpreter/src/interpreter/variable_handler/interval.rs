use crate::data::ast::{DoType, Expr, Function, IfStatement, Interval, ObjectType, RangeInterval};

pub fn interval_from_expr(expr: &Expr) -> Interval {
    match expr {
        Expr::Scope {
            range: RangeInterval { start, .. },
            ..
        } => *start,
        Expr::ComplexLiteral(_e, RangeInterval { start, .. }) => *start,
        Expr::MapExpr(_e, RangeInterval { start, .. }) => *start,
        Expr::VecExpr(_e, RangeInterval { start, .. }) => *start,
        Expr::ObjectExpr(fnexpr) => interval_from_reserved_fn(fnexpr),
        Expr::InfixExpr(_i, expr, _e) => interval_from_expr(expr), // RangeInterval ?
        Expr::PathExpr { literal, .. } => interval_from_expr(literal),
        Expr::ForEachExpr(_, _, _, _, RangeInterval { start, .. }) => *start,
        Expr::IdentExpr(ident) => ident.interval.to_owned(),
        Expr::LitExpr(literal) => literal.interval.to_owned(),
        Expr::IfExpr(ifstmt) => interval_from_if_stmt(ifstmt),
    }
}

pub fn interval_from_if_stmt(ifstmt: &IfStatement) -> Interval {
    match ifstmt {
        IfStatement::IfStmt { ref cond, .. } => interval_from_expr(cond),
        IfStatement::ElseStmt(_e, range) => range.start,
    }
}

pub fn interval_from_reserved_fn(reservedfn: &ObjectType) -> Interval {
    match reservedfn {
        ObjectType::Goto(_g, ident) => ident.interval.to_owned(),
        ObjectType::Use(expr) => interval_from_expr(expr),
        ObjectType::Do(DoType::Update(expr, ..)) => interval_from_expr(expr),
        ObjectType::Do(DoType::Exec(expr)) => interval_from_expr(expr),
        ObjectType::Say(expr) => interval_from_expr(expr),
        ObjectType::Remember(ident, ..) => ident.interval.to_owned(),
        ObjectType::Assign(ident, ..) => interval_from_expr(ident),
        ObjectType::As(ident, ..) => ident.interval.to_owned(),
        ObjectType::Import { step_name, .. } => step_name.interval.to_owned(),
        ObjectType::Normal(Function { interval, .. }) => interval.to_owned(),
        ObjectType::Hold(interval) => interval.to_owned(),
        ObjectType::Break(interval) => interval.to_owned(),
    }
}
