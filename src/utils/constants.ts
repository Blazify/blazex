export const DIGITS: (number | string)[] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, "."];

export const INT = "INT";
export const FLOAT = "FLOAT";
export const PLUS = "PLUS";
export const MINUS = "MINUS";
export const MULTIPLY = "MULTIPLY";
export const DIVIDE = "DIVIDE";
export const LEFT_PARENTHESIS = "LEFT_PARENTHESIS";
export const RIGHT_PARENTHESIS = "RIGHT_PARENTHESIS";
export const POWER = "POWER";
export const KEYWORD = "KEYWORD";
export const IDENTIFIER = "IDENTIFIER";
export const EQUALS = "EQUALS"
export const DOUBLE_EQUALS = "DOUBLE_EQUALS"
export const NOT_EQUALS = "NOT_EQUALS"
export const LESS_THAN_EQUALS = "LESS_THAN_EQUALS"
export const GREATER_THAN_EQUALS = "GREATER_THAN_EQUALS"
export const LESS_THAN = "LESS_THAN"
export const GREATER_THAN = "GREATER_THAN"
export const EOF = "EOF";

export type TYPES =
  | "INT" // int
  | "FLOAT" // float
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
  | "DOUBLE_EQUALS"
  | "NOT_EQUALS"
  | "LESS_THAN_EQUALS"
  | "GREATER_THAN_EQUALS"
  | "LESS_THAN"
  | "GREATER_THAN"
  | "EOF"; // end

export const ASCII_LETTERS_AND_DIGITS = "_0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".split("");
export const ASCII_LETTERS = "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".split("");

export const KEYWORDS = [
  "var",
  "and",
  "or",
  "not"
]