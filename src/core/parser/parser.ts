import { InvalidSyntaxError } from "../../error/invalidsyntax.ts";
import {
  DIVIDE,
  EOF,
  FLOAT,
  INT,
  MINUS,
  MULTIPLY,
  PLUS,
} from "../../utils/constants.ts";
import { BinOpNode } from "../node/binary_op_node.ts";
import { NumberNode } from "../node/number_nodes.ts";
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
    if(!res.error && this.currentToken.type !== EOF) {
      return res.failure(new InvalidSyntaxError(this.currentToken.positionStart!, this.currentToken.positionEnd!, "Expected '+' or '-' or '*' or '/'"))
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

  public factor(): ParseResult {
    const res = new ParseResult();
    const token = this.currentToken;
    if ([INT, FLOAT].includes(token.type)) {
      res.register(this.advance());
      return res.success(new NumberNode(token));
    }
    
    return res.failure(new InvalidSyntaxError(token.positionStart!, token.positionEnd!, "A Int or Float was Expected"))
  }

  public term(): ParseResult {
    const res = new ParseResult();
    let left: ParseResult | BinOpNode | NumberNode = res.register(this.factor()!)!;
    if(res.error) return res;

    while ([MULTIPLY, DIVIDE].includes(this.currentToken.type)) {
      const opToken = this.currentToken;
      res.register(this.advance());
      const right = res.register(this.factor()!);
      if(res.error) return res;
      left = new BinOpNode(left, opToken, right);
    }
    
    return res.success(left as BinOpNode | NumberNode);
  }

  public expr(): ParseResult {
    const res = new ParseResult();
    let left: ParseResult | BinOpNode | NumberNode = res.register(this.term()!)!;
    if(res.error) return res;

    while ([PLUS, MINUS].includes(this.currentToken.type)) {
      const opToken = this.currentToken;
      res.register(this.advance());
      const right = res.register(this.term()!);
      if(res.error) return res;
      left = new BinOpNode(left, opToken, right);
    }
    
    return res.success(left as BinOpNode | NumberNode);
  }

  public clone(): Parser {
    const parser = new Parser(this.tokens);
    parser.tokenIndex = this.tokenIndex;
    parser.currentToken = this.currentToken;
    return parser;
  }
}
