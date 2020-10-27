import {
  DIVIDE,
  FLOAT,
  INT,
  MINUS,
  MULTIPLY,
  PLUS,
} from "../utils/constants.ts";
import { BinOpNode } from "./node/binary_op_node.ts";
import { NumberNode } from "./node/number_nodes.ts";
import { Token } from "./token.ts";

export class Parser {
  public tokenIndex = -1;
  public currentToken!: Token;
  public constructor(public tokens: Token[]) {
    this.advance();
  }

  public parse(): BinOpNode {
    return this.expr();
  }

  public advance(): Token {
    this.tokenIndex += 1;
    if (this.tokenIndex < this.tokens.length) {
      this.currentToken = this.tokens[this.tokenIndex];
    }

    return this.currentToken;
  }

  public factor(): NumberNode | undefined {
    const token = this.currentToken;
    if ([INT, FLOAT].includes(token.type as string)) {
      this.advance();
      return new NumberNode(token);
    }
  }

  public term(): BinOpNode {
    let left = this.factor();
    let binop!: BinOpNode;

    while ([MULTIPLY, DIVIDE].includes(this.currentToken.type)) {
      const opToken = this.currentToken;
      this.advance();
      const right = this.factor();
      binop = new BinOpNode(left!, opToken, right!);
    }

    return binop;
  }

  public expr(): BinOpNode {
    let left: NumberNode = this.factor()!;
    let binop!: BinOpNode;

    while ([PLUS, MINUS].includes(this.currentToken.type)) {
      const opToken = this.currentToken;
      this.advance();
      const right = this.term();
      binop = new BinOpNode(left, opToken, right);
    }

    return binop;
  }
}
