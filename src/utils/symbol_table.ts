import { Number } from "../core/number.ts";

export class SymbolTable {
    public symbols: Map<string, Number> = new Map();
    public parent?: SymbolTable;


    public get(name: string) {
        const value = this.symbols.get(name);
        if(!value && this.parent) return this.parent?.symbols.get(name);
        return value;
    }

    public set(name: string, value: Number) {
        return this.symbols.set(name, value);
    }

    public delete(name: string) {
        return this.symbols.delete(name);
    } 
}