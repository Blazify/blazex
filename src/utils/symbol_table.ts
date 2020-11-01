import { BaseType } from "../core/types/base_type.ts";
import { Variable } from "./variable.ts";

export class SymbolTable {
  public symbols: Map<string, Variable<BaseType>> = new Map();
  public parent?: SymbolTable;

  public get(name: string): Variable<BaseType> {
    const value = this.symbols.get(name);
    if (!value && this.parent) return this.parent.get(name);
    return value!;
  }

  public set(name: string, value: Variable<any>) {
    return this.symbols.set(name, value);
  }

  public delete(name: string) {
    return this.symbols.delete(name);
  }
}
