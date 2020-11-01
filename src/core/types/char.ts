import { TYPES } from "../../utils/constants.ts";
import { BaseType } from "./base_type.ts";

export class Char extends BaseType {
    constructor(private _value: string) {
        super();
    }

    get type(): TYPES {
        return "Char"
    }

    get value() {
        return this._value;
    }

    public clone() {
        return new Char(this.value).setPosition(
          this.positionStart,
          this.positionEnd,
        ).setContext(this.context);
    }

    public represent() {
        return `${this.value}`
    }
}