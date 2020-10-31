import { BinOpNode } from "../core/node/binary_op_node.ts";
import { IfNode } from "../core/node/if_node.ts";
import { NumberNode } from "../core/node/number_nodes.ts";
import { UnaryOpNode } from "../core/node/unary_op_node.ts";
import { VarAcessNode } from "../core/node/var_access_node.ts";
import { VarAssignNode } from "../core/node/var_assign_node.ts";

export const DIGITS: (number | string)[] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, "."];

export const COLON = "COLON"
export const INT = "Int";
export const FLOAT = "Float";
export const PLUS = "PLUS";
export const MINUS = "MINUS";
export const MULTIPLY = "MULTIPLY";
export const DIVIDE = "DIVIDE";
export const LEFT_PARENTHESIS = "LEFT_PARENTHESIS";
export const RIGHT_PARENTHESIS = "RIGHT_PARENTHESIS";
export const POWER = "POWER";
export const KEYWORD = "KEYWORD";
export const IDENTIFIER = "IDENTIFIER";
export const EQUALS = "EQUALS";
export const DOUBLE_EQUALS = "DOUBLE_EQUALS";
export const NOT_EQUALS = "NOT_EQUALS";
export const LESS_THAN_EQUALS = "LESS_THAN_EQUALS";
export const GREATER_THAN_EQUALS = "GREATER_THAN_EQUALS";
export const LESS_THAN = "LESS_THAN";
export const GREATER_THAN = "GREATER_THAN";
export const EOF = "EOF";

export type TYPES =
  | "Int" // int
  | "Float" // float
  | "PLUS" // +
  | "MINUS" // -
  | "MULTIPLY" // *
  | "DIVIDE" // /
  | "LEFT_PARENTHESIS" // (
  | "RIGHT_PARENTHESIS" // )
  | "POWER" // ^
  | "KEYWORD" // var
  | "IDENTIFIER" // var_name
  | "EQUALS" // =
  | "DOUBLE_EQUALS" // ==
  | "NOT_EQUALS" // !=
  | "LESS_THAN_EQUALS" // <=
  | "GREATER_THAN_EQUALS" // >=
  | "LESS_THAN" // <
  | "GREATER_THAN" // >
  | "COLON"
  | "EOF"; // end

export const ASCII_LETTERS_AND_DIGITS =
  "_0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".split("");
export const ASCII_LETTERS =
  "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".split("");

export const KEYWORDS = [
  "val",
  "and", // &&
  "or", // ||
  "not", // !,
  "if",
  "then",
  "else",
];

export type Nodes =
  | BinOpNode
  | NumberNode
  | UnaryOpNode
  | VarAcessNode
  | VarAssignNode
  | IfNode;
