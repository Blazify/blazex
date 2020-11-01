import { InvalidSyntaxError } from "../../error/invalidsyntax.ts";
import { InvalidTypeError } from "../../error/typeerror.ts";
import {
  ARROW,
  COLON,
  COMMA,
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
  LangTypes,
  LEFT_PARENTHESIS,
  LESS_THAN,
  LESS_THAN_EQUALS,
  MINUS,
  MULTIPLY,
  Nodes,
  NOT_EQUALS,
  PLUS,
  POWER,
  RIGHT_PARENTHESIS,
  TYPES,
} from "../../utils/constants.ts";
import { BinOpNode } from "../node/binary_op_node.ts";
import { CallNode } from "../node/call_node.ts";
import { ForNode } from "../node/for_node.ts";
import { FuncDefNode } from "../node/func_def.ts";
import { IfNode } from "../node/if_node.ts";
import { NumberNode } from "../node/number_nodes.ts";
import { UnaryOpNode } from "../node/unary_op_node.ts";
import { VarAcessNode } from "../node/var_access_node.ts";
import { VarAssignNode } from "../node/var_assign_node.ts";
import { WhileNode } from "../node/while_node.ts";
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
    if (
      !res.error && this.currentToken.type !== EOF &&
      this.currentToken.type !== INT
    ) {
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
    } else if (this.currentToken.match(KEYWORD, "for")) {
      const forExpr = res.register(this.forExpr());
      if (res.error) return res;
      return res.success(forExpr);
    } else if (this.currentToken.match(KEYWORD, "while")) {
      const whileExpr = res.register(this.whileExpr());
      if (res.error) return res;
      return res.success(whileExpr);
    } else if (this.currentToken.match(KEYWORD, "fun")) {
      const funDef = res.register(this.funDef());
      if (res.error) return res;
      return res.success(funDef);
    }

    return res.failure(
      new InvalidSyntaxError(
        token.positionStart!,
        token.positionEnd!,
        "A Int or Float or Identifier, '+', '-', '(' was Expected",
      ),
    );
  }

  public call(): ParseResult {
    const res = new ParseResult();
    const atom = res.register(this.atom());
    if (res.error) return res;
    const argNodes: Nodes[] = [];

    if (this.currentToken.type === LEFT_PARENTHESIS) {
      res.registerAdvancement();
      this.advance();
      // @ts-expect-error
      if (this.currentToken.type === RIGHT_PARENTHESIS) {
        res.registerAdvancement();
        this.advance();
      } else {
        argNodes.push(res.register(this.expr()));
        if (res.error) {
          return res.failure(
            new InvalidSyntaxError(
              this.currentToken.positionStart!,
              this.currentToken.positionEnd!,
              "Expected ')', 'var', int, float, identifier, '+', '-' or '('",
            ),
          );
        }

        // @ts-expect-error
        while (this.currentToken.type === COMMA) {
          res.registerAdvancement();
          this.advance();
          argNodes.push(res.register(this.expr()));
        }

        // @ts-expect-error
        if (this.currentToken.type !== RIGHT_PARENTHESIS) {
          return res.failure(
            new InvalidSyntaxError(
              this.currentToken.positionStart!,
              this.currentToken.positionEnd!,
              "Expected ')' or ','",
            ),
          );
        }
      }
      return res.success(new CallNode(atom, argNodes));
    }

    return res.success(atom);
  }

  public funDef(): ParseResult {
    const res = new ParseResult();
    if (!(this.currentToken.match(KEYWORD, "fun"))) {
      return res.failure(
        new InvalidSyntaxError(
          this.currentToken.positionStart!,
          this.currentToken.positionEnd!,
          "Expected 'fun' keyword",
        ),
      );
    }

    res.registerAdvancement();
    this.advance();

    let varNameToken = undefined;
    if (this.currentToken.type === IDENTIFIER) {
      varNameToken = this.currentToken;
      res.registerAdvancement();
      this.advance();

      // @ts-expect-error
      if (this.currentToken.type !== LEFT_PARENTHESIS) {
        return res.failure(
          new InvalidSyntaxError(
            this.currentToken.positionStart!,
            this.currentToken.positionEnd!,
            "Expected '('",
          ),
        );
      }
    } else {
      if (this.currentToken.type !== LEFT_PARENTHESIS) {
        return res.failure(
          new InvalidSyntaxError(
            this.currentToken.positionStart!,
            this.currentToken.positionEnd!,
            "Expected '(' or identifier",
          ),
        );
      }
    }

    res.registerAdvancement();
    this.advance();

    const argNameTokens: Token[] = [];

    // @ts-expect-error
    if (this.currentToken.type === IDENTIFIER) {
      const name = this.currentToken;

      res.registerAdvancement();
      this.advance();

      if (this.currentToken.type === COLON) {
        res.registerAdvancement();
        this.advance();

        if (LangTypes.includes(this.currentToken.value as any)) {
          name.type = this.currentToken.value as TYPES;
        } else {
          return res.failure(
            new InvalidTypeError(
              this.currentToken.positionStart!,
              this.currentToken.positionEnd!,
              "Unknown Type",
            ),
          );
        }
        argNameTokens.push(name);

        res.registerAdvancement();
        this.advance();
      }

      while (this.currentToken.type === COMMA) {
        res.registerAdvancement();
        this.advance();

        if (this.currentToken.type === IDENTIFIER) {
          const nameE = this.currentToken;

          res.registerAdvancement();
          this.advance();

          if (this.currentToken.type === COLON) {
            res.registerAdvancement();
            this.advance();

            if (LangTypes.includes(this.currentToken.value as any)) {
              nameE.type = this.currentToken.value as TYPES;
            } else {
              return res.failure(
                new InvalidTypeError(
                  this.currentToken.positionStart!,
                  this.currentToken.positionEnd!,
                  "Unknown Type",
                ),
              );
            }

            argNameTokens.push(nameE);

            res.registerAdvancement();
            this.advance();
          }
        } else {
          return res.failure(
            new InvalidSyntaxError(
              this.currentToken.positionStart!,
              this.currentToken.positionEnd!,
              "Expected Identifier",
            ),
          );
        }
      }

      if (this.currentToken.type !== RIGHT_PARENTHESIS) {
        return res.failure(
          new InvalidSyntaxError(
            this.currentToken.positionStart!,
            this.currentToken.positionEnd!,
            "Expected ')' or ','",
          ),
        );
      }
    } else {
      // @ts-expect-error
      if (this.currentToken.type !== RIGHT_PARENTHESIS) {
        return res.failure(
          new InvalidSyntaxError(
            this.currentToken.positionStart!,
            this.currentToken.positionEnd!,
            "Expected ')' or identifier",
          ),
        );
      }
    }

    res.registerAdvancement();
    this.advance();

    if (this.currentToken.type !== ARROW) {
      return res.failure(
        new InvalidSyntaxError(
          this.currentToken.positionStart!,
          this.currentToken.positionEnd!,
          "Expected '=>'",
        ),
      );
    }

    res.registerAdvancement();
    this.advance();

    const body = res.register(this.expr());
    if (res.error) return res;

    return res.success(new FuncDefNode(body, varNameToken, argNameTokens));
  }

  public forExpr(): ParseResult {
    const res = new ParseResult();

    if (!(this.currentToken.match(KEYWORD, "for"))) {
      return res.failure(
        new InvalidSyntaxError(
          this.currentToken.positionStart!,
          this.currentToken.positionEnd!,
          "Expected for keyword",
        ),
      );
    }

    res.registerAdvancement();
    this.advance();

    if (this.currentToken.type !== IDENTIFIER) {
      return res.failure(
        new InvalidSyntaxError(
          this.currentToken.positionStart!,
          this.currentToken.positionEnd!,
          "Expected Identifier",
        ),
      );
    }

    const varName = this.currentToken;
    res.registerAdvancement();
    this.advance();

    let startValue: Nodes;
    let type: TYPES;
    // @ts-expect-error
    if (this.currentToken.type !== EQUALS) {
      return res.failure(
        new InvalidSyntaxError(
          this.currentToken.positionStart!,
          this.currentToken.positionEnd!,
          "Expected ':' or '='",
        ),
      );
    } else {
      res.registerAdvancement();
      this.advance();

      startValue = res.register(this.expr());
      if (res.error) return res;
      if (LangTypes.includes(startValue.type as any)) {
        type = startValue.type;
      } else {
        return res.failure(
          new InvalidTypeError(
            this.currentToken.positionStart!,
            this.currentToken.positionEnd!,
            "Unknown type",
          ),
        );
      }
    }

    if (!this.currentToken.match(KEYWORD, "to")) {
      return res.failure(
        new InvalidSyntaxError(
          this.currentToken.positionStart!,
          this.currentToken.positionEnd!,
          "Expected 'to' keyword",
        ),
      );
    }

    res.registerAdvancement();
    this.advance();

    const endValue = res.register(this.expr());
    if (res.error) return res;

    if (endValue.type !== type) {
      return res.failure(
        new InvalidTypeError(
          this.currentToken.positionStart!,
          this.currentToken.positionEnd!,
          `${endValue.type} doesn't satsfy ${type}`,
        ),
      );
    }

    let step: Nodes | undefined;

    if (this.currentToken.match(KEYWORD, "step")) {
      res.registerAdvancement();
      this.advance();

      step = res.register(this.expr());
    } else {
      step = undefined;
    }

    if (!(this.currentToken.match(KEYWORD, "then"))) {
      return res.failure(
        new InvalidSyntaxError(
          this.currentToken.positionStart!,
          this.currentToken.positionEnd!,
          "Expected 'then' keyword",
        ),
      );
    }

    res.registerAdvancement();
    this.advance();

    const body = res.register(this.expr());
    if (res.error) return res;

    return res.success(new ForNode(varName, startValue, endValue, body, step));
  }

  public whileExpr(): ParseResult {
    const res = new ParseResult();

    if (!this.currentToken.match(KEYWORD, "while")) {
      return res.failure(
        new InvalidSyntaxError(
          this.currentToken.positionStart!,
          this.currentToken.positionEnd!,
          "Expected 'while' keyword",
        ),
      );
    }

    res.registerAdvancement();
    this.advance();

    const condition = res.register(this.expr());
    if (res.error) return res;

    if (!this.currentToken.match(KEYWORD, "then")) {
      return res.failure(
        new InvalidSyntaxError(
          this.currentToken.positionStart!,
          this.currentToken.positionEnd!,
          "Expected 'then' keyword",
        ),
      );
    }

    res.registerAdvancement();
    this.advance();

    const body = res.register(this.expr());
    if (res.error) return res;

    return res.success(new WhileNode(condition, body));
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

  public power(): ParseResult {
    const res = new ParseResult();
    let left = res.register(
      this.call()!,
    )!;
    if (res.error) return res;

    while (this.currentToken.type === POWER) {
      const opToken = this.currentToken;
      res.registerAdvancement();
      this.advance();
      const right = res.register(this.factor()!);
      if (res.error) return res;
      if (right.type === IDENTIFIER || left.type === IDENTIFIER) {
      } else if (right.type !== left.type) {
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
      if (right.type === IDENTIFIER || left.type === IDENTIFIER) {
      } else if (right.type !== left.type) {
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

        let expr = res.register(this.expr());
        if (res.error) return res;
        if (LangTypes.includes(expr.type as any)) {
          return res.success(
            new VarAssignNode(varName, expr, expr.type, false),
          );
        } else {
          res.failure(
            new InvalidTypeError(
              expr.positionStart,
              expr.positionEnd,
              "Unknown Type, Please Specify a type",
            ),
          );
        }
      }

      res.registerAdvancement();
      this.advance();

      const type = this.currentToken;
      if (!LangTypes.includes(type.value as any)) {
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
    } else if (this.currentToken.match(KEYWORD, "var")) {
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

        let expr = res.register(this.expr());
        if (res.error) return res;
        if (LangTypes.includes(expr.type as any)) {
          return res.success(
            new VarAssignNode(varName, expr, expr.type, false),
          );
        } else {
          res.failure(
            new InvalidTypeError(
              expr.positionStart,
              expr.positionEnd,
              "Unknown Type, Please Specify a type",
            ),
          );
        }
      }

      res.registerAdvancement();
      this.advance();

      const type = this.currentToken;
      if (!LangTypes.includes(type.value as any)) {
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
        new VarAssignNode(varName, expr, type.value! as TYPES, true),
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
      if (right.type === IDENTIFIER || left.type === IDENTIFIER) {
      } else if (right.type !== left.type) {
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
      if (right.type === IDENTIFIER || left.type === IDENTIFIER) {
      } else if (right.type !== left.type) {
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
      if (right.type === IDENTIFIER || left.type === IDENTIFIER) {
      } else if (right.type !== left.type) {
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
