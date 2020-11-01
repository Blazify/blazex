import { Err } from "../../error/err.ts";
import { RuntimeError } from "../../error/runtimeerr.ts";
import { BaseType } from "./base_type.ts";

export class Number extends BaseType {
  public get value(): number {
    return this._value;
  }

  public set value(value: number) {
    this._value = value;
  }

  constructor(private _value: number) {
    super();
  }

  public addTo(
    other: Number,
  ): { result: Number | null; error: Err | null } {
    if (other instanceof Number) {
      return {
        result: new Number(this.value + other.value).setContext(this.context),
        error: null,
      };
    } else {
      return super.addTo(other);
    }
  }

  public subBy(
    other: Number,
  ): { result: Number | null; error: Err | null } {
    if (other instanceof Number) {
      return {
        result: new Number(this.value - other.value).setContext(this.context),
        error: null,
      };
    } else {
      return super.subBy(other);
    }
  }

  public multiBy(
    other: Number,
  ): { result: Number | null; error: Err | null } {
    if (other instanceof Number) {
      return {
        result: new Number(this.value * other.value).setContext(this.context),
        error: null,
      };
    } else {
      return super.multiBy(other);
    }
  }

  public divBy(
    other: Number,
  ): { result: Number | null; error: Err | null } {
    if (other instanceof Number) {
      if (other.value == 0) {
        return {
          result: null,
          error: new RuntimeError(
            other.positionStart!,
            other.positionEnd!,
            "Division by 0 isn't possible",
            this.context!,
          ),
        };
      }
      return {
        result: new Number(this.value / other.value).setContext(this.context),
        error: null,
      };
    } else {
      return super.divBy(other);
    }
  }

  public powBy(
    other: Number,
  ): { result: Number | null; error: Err | null } {
    if (other instanceof Number) {
      return {
        result: new Number(this.value ** other.value).setContext(this.context),
        error: null,
      };
    } else {
      return super.powBy(other);
    }
  }

  public equals(
    other: Number,
  ): { result: Number | null; error: Err | null } {
    if (other instanceof Number) {
      return {
        result: new Number(this.numberToBoolean(this.value === other.value))
          .setContext(this.context),
        error: null,
      };
    } else {
      return super.equals(other);
    }
  }

  public notEquals(
    other: Number,
  ): { result: Number | null; error: Err | null } {
    if (other instanceof Number) {
      return {
        result: new Number(this.numberToBoolean(this.value !== other.value))
          .setContext(this.context),
        error: null,
      };
    } else {
      return super.notEquals(other);
    }
  }

  public lessThan(
    other: Number,
  ): { result: Number | null; error: Err | null } {
    if (other instanceof Number) {
      return {
        result: new Number(this.numberToBoolean(this.value < other.value))
          .setContext(this.context),
        error: null,
      };
    } else {
      return super.lessThan(other);
    }
  }

  public greaterThan(
    other: Number,
  ): { result: Number | null; error: Err | null } {
    if (other instanceof Number) {
      return {
        result: new Number(this.numberToBoolean(this.value > other.value))
          .setContext(this.context),
        error: null,
      };
    } else {
      return super.greaterThan(other);
    }
  }

  public lessThanEquals(
    other: Number,
  ): { result: Number | null; error: Err | null } {
    if (other instanceof Number) {
      return {
        result: new Number(this.numberToBoolean(this.value <= other.value))
          .setContext(this.context),
        error: null,
      };
    } else {
      return super.lessThanEquals(other);
    }
  }

  public greaterThanEquals(
    other: Number,
  ): { result: Number | null; error: Err | null } {
    if (other instanceof Number) {
      return {
        result: new Number(this.numberToBoolean(this.value >= other.value))
          .setContext(this.context),
        error: null,
      };
    } else {
      return super.greaterThanEquals(other);
    }
  }

  public and(
    other: Number,
  ): { result: Number | null; error: Err | null } {
    if (other instanceof Number) {
      return {
        result: new Number(
          this.numberToBoolean(Boolean(this.value && other.value)),
        ).setContext(this.context),
        error: null,
      };
    } else {
      return super.and(other);
    }
  }

  public or(
    other: Number,
  ): { result: Number | null; error: Err | null } {
    if (other instanceof Number) {
      return {
        result: new Number(
          this.numberToBoolean(Boolean(this.value || other.value)),
        ).setContext(this.context),
        error: null,
      };
    } else {
      return super.or(other);
    }
  }

  public not(): { result: Number | null; error: Err | null } {
    return {
      result: new Number(this.numberToBoolean(Boolean(!this.value))).setContext(
        this.context,
      ),
      error: null,
    };
  }

  public clone() {
    return new Number(this.value).setPosition(
      this.positionStart,
      this.positionEnd,
    ).setContext(this.context);
  }

  public numberToBoolean(boo: boolean): number {
    if (boo == true) return 1;
    return 0;
  }

  public get type(): "Int" | "Float" {
    return this.value.toString().includes(".") ? "Float" : "Int";
  }

  public represent() {
    return `${this.value}`;
  }
}
