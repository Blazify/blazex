import { InvalidSyntaxError } from "../../error/invalidsyntax.ts";
import {
  DIVIDE,
  DOUBLE_EQUALS,
  EOF,
  EQUALS,
  FLOAT,
  GREATER_THAN,
  GREATER_THAN_EQUALS,
  IDENTIFIER,
  INT,
  KEYWORD,
  LESS_THAN,
  LESS_THAN_EQUALS,
  MINUS,
  MULTIPLY,
  NOT_EQUALS,
  PLUS,
  POWER,
} from "../../utils/constants.ts";
import { BinOpNode } from "../node/binary_op_node.ts";
import { NumberNode } from "../node/number_nodes.ts";
import { UnaryOpNode } from "../node/unary_op_node.ts";
import { VarAcessNode } from "../node/var_access_node.ts";
import { VarAssignNode } from "../node/var_assign_node.ts";
import { Token } from "../token.ts";
import { ParseResult } from "./parse_result.ts";

export class Parser {
  public tokenIndex = -1;
  public currentToken!: Token;
  public constructor(public tokens: Token[]) {
    this.advance();
  }

  public parse() {
    const res = this.expr();
    if (!res.error && this.currentToken.type !== EOF) {
      return res.failure(
        new InvalidSyntaxError(
          this.currentToken.positionStart!,
          this.currentToken.positionEnd!,
          "Expected '+' or '-' or '*' or '/'",
        ),
      );
    }

    return res;
  }

  public advance(): Token {
    this.tokenIndex += 1;
    if (this.tokenIndex < this.tokens.length) {
      this.currentToken = this.tokens[this.tokenIndex];
    }

    return this.currentToken;
  }

  public atom(): ParseResult {
    const res = new ParseResult();
    const token = this.currentToken;

    if ([INT, FLOAT].includes(token.type)) {
      res.registerAdvancement();
      this.advance();
      return res.success(new NumberNode(token));
    } else if (token.type === IDENTIFIER) {
      res.registerAdvancement();
      this.advance();
      return res.success(new VarAcessNode(token));
    } else if (token.type === "LEFT_PARENTHESIS") {
      res.registerAdvancement();
      this.advance();
      const expr = res.register(this.expr());
      if (res.error) return res;
      if (this.currentToken.type === "RIGHT_PARENTHESIS") {
        res.registerAdvancement();
        this.advance();
        return res.success(expr as BinOpNode);
      } else {
        return res.failure(
          new InvalidSyntaxError(
            this.currentToken.positionStart!,
            this.currentToken.positionEnd!,
            "Expected ')' but found none!",
          ),
        );
      }
    }

    return res.failure(
      new InvalidSyntaxError(
        token.positionStart!,
        token.positionEnd!,
        "A Int or Float or Identifier, '+', '-', '(' was Expected",
      ),
    );
  }

  public power(): ParseResult {
    const res = new ParseResult();
    let left = res.register(
      this.atom()!,
    )!;
    if (res.error) return res;

    while (this.currentToken.type === POWER) {
      const opToken = this.currentToken;
      res.registerAdvancement();
      this.advance();
      const right = res.register(this.factor()!);
      if (res.error) return res;
      left = new BinOpNode(left, opToken, right);
    }

    return res.success(left);
  }

  public factor(): ParseResult {
    const res = new ParseResult();
    const token = this.currentToken;

    if ([PLUS, MINUS].includes(token.type)) {
      res.registerAdvancement();
      this.advance();
      const fac = res.register(this.factor());
      if (!fac) {
        return res.failure(
          new InvalidSyntaxError(
            token.positionStart!,
            this.currentToken.positionEnd!,
            "Expected A Number after a Unary Operator",
          ),
        );
      }
      if (res.error) return res;
      return res.success(new UnaryOpNode(token, fac));
    }

    return this.power();
  }

  public term(): ParseResult {
    const res = new ParseResult();
    let left = res.register(
      this.factor()!,
    )!;
    if (res.error) return res;

    while ([MULTIPLY, DIVIDE].includes(this.currentToken.type)) {
      const opToken = this.currentToken;
      res.registerAdvancement();
      this.advance();
      const right = res.register(this.factor()!);
      if (res.error) return res;
      left = new BinOpNode(left, opToken, right);
    }

    return res.success(left);
  }

  public expr(): ParseResult {
    const res = new ParseResult();
    if (this.currentToken.match(KEYWORD, "var")) {
      res.registerAdvancement();
      this.advance();

      if (this.currentToken.type !== IDENTIFIER) {
        return res.failure(
          new InvalidSyntaxError(
            this.currentToken.positionStart!,
            this.currentToken.positionEnd!,
            "Expected Identifier after Keyword",
          ),
        );
      }

      const varName = this.currentToken;
      res.registerAdvancement();
      this.advance();

      // @ts-expect-error // due to some stupid reasons vscode vomits error at me (-,-)
      if (this.currentToken.type !== EQUALS) {
        return res.failure(
          new InvalidSyntaxError(
            this.currentToken.positionStart!,
            this.currentToken.positionEnd!,
            "Expected '='",
          ),
        );
      }

      res.registerAdvancement();
      this.advance();

      const expr = res.register(this.expr());
      if (res.error) return res;
      return res.success(new VarAssignNode(varName, expr));
    }

    let left = res.register(
      this.compExpr(),
    )!;
    if (res.error) return res;

    while (this.currentToken.match(KEYWORD, "and") || this.currentToken.match(KEYWORD, "or")) {
      const opToken = this.currentToken;
      res.registerAdvancement();
      this.advance();
      const right = res.register(this.compExpr());
      if (res.error) return res;
      left = new BinOpNode(left, opToken, right);
    }

    left = res.register(left);

    if (res.error) {
      return res.failure(
        new InvalidSyntaxError(
          this.currentToken.positionStart!,
          this.currentToken.positionEnd!,
          "Expected 'var', int, float, identifier, '+', '-' or '('",
        ),
      );
    }

    return res.success(left);
  }

  public compExpr(): ParseResult {
    const res = new ParseResult()

    if(this.currentToken.match(KEYWORD, "not")) {
      const opToken = this.currentToken;
      res.registerAdvancement();
      this.advance();

      const node = res.register(this.compExpr());
      if(res.error) return res;

      return res.success(new UnaryOpNode(opToken, node))
    }

    let left = res.register(
      this.arithExpr(),
    )!;
    if (res.error) return res;

    while ([DOUBLE_EQUALS, NOT_EQUALS, LESS_THAN, GREATER_THAN, LESS_THAN_EQUALS, GREATER_THAN_EQUALS].includes(this.currentToken.type)) {
      const opToken = this.currentToken;
      res.registerAdvancement();
      this.advance();
      const right = res.register(this.arithExpr());
      if (res.error) return res;
      left = new BinOpNode(left, opToken, right);
    }

    const node = res.register(left);
    if(res.error) return res.failure(new InvalidSyntaxError(node.positionStart, node.positionEnd, "A Int or Float or Identifier, '+', '-', '(', 'not', '!' was Expected"));

    return res.success(node);
  }

  public arithExpr(): ParseResult {
    const res = new ParseResult();
    let left = res.register(
      this.term(),
    )!;
    if (res.error) return res;

    while ([PLUS, MINUS].includes(this.currentToken.type)) {
      const opToken = this.currentToken;
      res.registerAdvancement();
      this.advance();
      const right = res.register(this.term());
      if (res.error) return res;
      left = new BinOpNode(left, opToken, right);
    }

    return res.success(left);
  }
}
