import { Err } from "../error/err.ts";
import { Position } from "../error/position.ts";
import { RuntimeError } from "../error/runtimeerr.ts";
import { Context } from "./context.ts";

export class Number {
  public positionStart!: Position | null;
  public positionEnd!: Position | null;
  public context!: Context | null;
  constructor(public value: number) {
    this.setPosition();
    this.setContext();
  }

  public setPosition(
    start: Position | null = null,
    end: Position | null = null,
  ) {
    this.positionStart = start;
    this.positionEnd = end;
    return this;
  }

  public addTo(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(this.value + other.value).setContext(this.context),
        error: null,
      };
    }
  }

  public subBy(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(this.value - other.value).setContext(this.context),
        error: null,
      };
    }
  }

  public multiBy(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(this.value * other.value).setContext(this.context),
        error: null,
      };
    }
  }

  public divBy(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
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
    }
  }

  public powBy(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(this.value ** other.value).setContext(this.context),
        error: null,
      };
    }
  }

  public equals(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(this.numberToBoolean(this.value === other.value))
          .setContext(this.context),
        error: null,
      };
    }
  }

  public notEquals(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(this.numberToBoolean(this.value !== other.value))
          .setContext(this.context),
        error: null,
      };
    }
  }

  public lessThan(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(this.numberToBoolean(this.value < other.value))
          .setContext(this.context),
        error: null,
      };
    }
  }

  public greaterThan(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(this.numberToBoolean(this.value > other.value))
          .setContext(this.context),
        error: null,
      };
    }
  }

  public lessThanEquals(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(this.numberToBoolean(this.value <= other.value))
          .setContext(this.context),
        error: null,
      };
    }
  }

  public greaterThanEquals(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(this.numberToBoolean(this.value >= other.value))
          .setContext(this.context),
        error: null,
      };
    }
  }

  public and(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(
          this.numberToBoolean(Boolean(this.value && other.value)),
        ).setContext(this.context),
        error: null,
      };
    }
  }

  public or(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(
          this.numberToBoolean(Boolean(this.value || other.value)),
        ).setContext(this.context),
        error: null,
      };
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

  public setContext(context: Context | null = null) {
    this.context = context;
    return this;
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

  public represent() {
    return `${this.value}`;
  }
}
