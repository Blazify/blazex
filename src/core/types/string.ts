import { Err } from "../../error/err.ts";
import { TYPES } from "../../utils/constants.ts";
import { BaseType } from "./base_type.ts";

export class String extends BaseType {
    constructor(private _value: string) {
        super();
    }

    get type(): TYPES {
        return "String"
    }

    get value() {
        return this._value;
    }

    public addTo(other: String): { result: String | null, error: Err | null } {
        if(other instanceof String) {
            return { result: new String(this.value + other.value).setContext(this.context), error: null }
        } else {
            return super.addTo(other);
        }
    }

    public clone() {
        return new String(this.value).setPosition(
          this.positionStart,
          this.positionEnd,
        ).setContext(this.context);
    }

    public represent() {
        return `${this.value}`
    }
}