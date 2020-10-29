import { Position } from "../error/position.ts";
import { SymbolTable } from "../utils/symbol_table.ts";

export class Context {
  public symbolTable?: SymbolTable
  constructor(
    public displayName: string,
    public parent?: Context,
    public parentEntryPosition?: Position,
  ) {}
}
