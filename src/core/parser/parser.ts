import { InvalidSyntaxError } from "../../error/invalidsyntax.ts";
import { InvalidTypeError } from "../../error/typeerror.ts";
import {
  COLON,
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
  Nodes,
  NOT_EQUALS,
  PLUS,
  POWER,
  TYPES,
} from "../../utils/constants.ts";
import { BinOpNode } from "../node/binary_op_node.ts";
import { IfNode } from "../node/if_node.ts";
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
    this.advance();
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
    } else if (this.currentToken.match(KEYWORD, "if")) {
      const ifExpr = res.register(this.ifExpr());
      if (res.error) return res;
      return res.success(ifExpr);
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
      if (right.type !== left.type) {
        return res.failure(
          new InvalidTypeError(
            left.positionStart,
            right.positionStart,
            `The Lefthand type of binary operation ${left.type} is not same as the one of ${right.type}`,
          ),
        );
      }
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
      if (right.type !== left.type) {
        return res.failure(
          new InvalidTypeError(
            left.positionStart,
            right.positionStart,
            `The Lefthand type of binary operation ${left.type} is not same as the one of ${right.type}`,
          ),
        );
      }
      left = new BinOpNode(left, opToken, right);
    }

    return res.success(left);
  }

  public ifExpr(): ParseResult {
    const res = new ParseResult();
    const cases: [Nodes, Nodes][] = [];
    let elseCase: Nodes | null = null;

    if (!(this.currentToken.match(KEYWORD, "if"))) {
      return res.failure(
        new InvalidSyntaxError(
          this.currentToken.positionStart!,
          this.currentToken.positionEnd!,
          "Expected if keyword",
        ),
      );
    }

    res.registerAdvancement();
    this.advance();

    const condition = res.register(this.expr());
    if (res.error) return res;

    if (!(this.currentToken.match(KEYWORD, "then"))) {
      return res.failure(
        new InvalidSyntaxError(
          this.currentToken.positionStart!,
          this.currentToken.positionEnd!,
          "Expected then keyword after if",
        ),
      );
    }

    res.registerAdvancement();
    this.advance();

    const expr = res.register(this.expr());
    if (res.error) return res;
    const type = expr.type;
    cases.push([condition, expr]);

    while (this.currentToken.match(KEYWORD, "else")) {
      res.registerAdvancement();
      this.advance();

      if (this.currentToken.match(KEYWORD, "if")) {
        res.registerAdvancement();
        this.advance();

        const ifElseCondition = res.register(this.expr());
        if (res.error) return res;

        if (!(this.currentToken.match(KEYWORD, "then"))) {
          return res.failure(
            new InvalidSyntaxError(
              this.currentToken.positionStart!,
              this.currentToken.positionEnd!,
              "Expected then keyword after if",
            ),
          );
        }
        res.registerAdvancement();
        this.advance();

        const ifElseExpr = res.register(this.expr());
        if (ifElseExpr.type !== type) {
          return res.failure(
            new InvalidTypeError(
              ifElseCondition.positionStart,
              ifElseCondition.positionEnd,
              `${ifElseExpr.type} doesn't satisfy ${type}`,
            ),
          );
        }
        cases.push([ifElseCondition, ifElseExpr]);
      } else {
        const elseExpr = res.register(this.expr());
        if (res.error) return res;
        if (elseExpr.type !== type) {
          return res.failure(
            new InvalidTypeError(
              elseExpr.positionStart,
              elseExpr.positionEnd,
              `${elseExpr.type} doesn't satisfy ${type}`,
            ),
          );
        }

        elseCase = elseExpr;
        break;
      }
    }

    return res.success(new IfNode(cases, elseCase, type));
  }

  public expr(): ParseResult {
    const res = new ParseResult();
    if (this.currentToken.match(KEYWORD, "val")) {
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
      if (this.currentToken.type !== COLON) {
        //@ts-expect-error
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
        return res.success(new VarAssignNode(varName, expr, expr.type, false));
      }

      res.registerAdvancement();
      this.advance();

      const type = this.currentToken;
      if (!(type.match(IDENTIFIER, INT) || type.match(IDENTIFIER, FLOAT))) {
        return res.failure(
          new InvalidTypeError(
            type.positionStart!,
            type.positionEnd!,
            "Unknown Type",
          ),
        );
      }

      res.registerAdvancement();
      this.advance();

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
      if (
        (expr.type == IDENTIFIER) ? expr.type : type.value !== type.value
      ) {
        return res.failure(
          new InvalidTypeError(
            varName.positionStart!,
            this.currentToken.positionEnd!,
            `${expr.type} is not a type of ${type.value}`,
          ),
        );
      }
      return res.success(
        new VarAssignNode(varName, expr, type.value! as TYPES, false),
      );
    }

    let left = res.register(
      this.compExpr(),
    )!;
    if (res.error) return res;

    while (
      this.currentToken.match(KEYWORD, "and") ||
      this.currentToken.match(KEYWORD, "or")
    ) {
      const opToken = this.currentToken;
      res.registerAdvancement();
      this.advance();
      const right = res.register(this.compExpr());
      if (res.error) return res;
      if (right.type !== left.type) {
        return res.failure(
          new InvalidTypeError(
            left.positionStart,
            right.positionStart,
            `The Lefthand type of binary operation ${left.type} is not same as the one of ${right.type}`,
          ),
        );
      }
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
    const res = new ParseResult();

    if (this.currentToken.match(KEYWORD, "not")) {
      const opToken = this.currentToken;
      res.registerAdvancement();
      this.advance();

      const node = res.register(this.compExpr());
      if (res.error) return res;

      return res.success(new UnaryOpNode(opToken, node));
    }

    let left = res.register(
      this.arithExpr(),
    )!;
    if (res.error) return res;

    while (
      [
        DOUBLE_EQUALS,
        NOT_EQUALS,
        LESS_THAN,
        GREATER_THAN,
        LESS_THAN_EQUALS,
        GREATER_THAN_EQUALS,
      ].includes(this.currentToken.type)
    ) {
      const opToken = this.currentToken;
      res.registerAdvancement();
      this.advance();
      const right = res.register(this.arithExpr());
      if (res.error) return res;
      if (right.type !== left.type) {
        return res.failure(
          new InvalidTypeError(
            left.positionStart,
            right.positionStart,
            `The Lefthand type of binary operation ${left.type} is not same as the one of ${right.type}`,
          ),
        );
      }
      left = new BinOpNode(left, opToken, right);
    }

    const node = res.register(left);
    if (res.error) {
      return res.failure(
        new InvalidSyntaxError(
          node.positionStart,
          node.positionEnd,
          "A Int or Float or Identifier, '+', '-', '(', 'not', '!' was Expected",
        ),
      );
    }

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
      if (right.type !== left.type) {
        return res.failure(
          new InvalidTypeError(
            left.positionStart,
            right.positionStart,
            `The Lefthand type of binary operation ${left.type} is not same as the one of ${right.type}`,
          ),
        );
      }
      left = new BinOpNode(left, opToken, right);
    }

    return res.success(left);
  }
}
