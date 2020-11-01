import { Variable } from "./variable.ts";

export class SymbolTable {
  public symbols: Map<string, Variable<any>> = new Map();
  public parent?: SymbolTable;

  public get<K = any>(name: string): Variable<K> | undefined {
    const value = this.symbols.get(name);
    if (!value && this.parent) return this.parent.get(name);
    return value;
  }

  public set(name: string, value: Variable<any>) {
    return this.symbols.set(name, value);
  }

  public delete(name: string) {
    return this.symbols.delete(name);
  }
}
