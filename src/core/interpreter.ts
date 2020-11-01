import { Err } from "../error/err.ts";
import { RuntimeError } from "../error/runtimeerr.ts";
import {
  DIVIDE,
  DOUBLE_EQUALS,
  GREATER_THAN,
  GREATER_THAN_EQUALS,
  KEYWORD,
  LESS_THAN,
  LESS_THAN_EQUALS,
  MINUS,
  MULTIPLY,
  Nodes,
  NOT_EQUALS,
  PLUS,
  POWER,
} from "../utils/constants.ts";
import { Variable } from "../utils/variable.ts";
import { Context } from "./context.ts";
import { BinOpNode } from "./node/binary_op_node.ts";
import { ForNode } from "./node/for_node.ts";
import { IfNode } from "./node/if_node.ts";
import { NumberNode } from "./node/number_nodes.ts";
import { UnaryOpNode } from "./node/unary_op_node.ts";
import { VarAcessNode } from "./node/var_access_node.ts";
import { VarAssignNode } from "./node/var_assign_node.ts";
import { WhileNode } from "./node/while_node.ts";
import { Number as MyNumber } from "./types/number.ts";
import { RuntimeResult } from "./runtime_result.ts";
import { FuncDefNode } from "./node/func_def.ts";
import { CallNode } from "./node/call_node.ts";
import { BaseType } from "./types/base_type.ts";
import { Function } from "./types/function.ts";

export class Interpreter {
  public visit(
    node: Nodes,
    context: Context,
  ): RuntimeResult {
    if (node instanceof BinOpNode) {
      return this.visitBinOpNode(node, context);
    } else if (node instanceof NumberNode) {
      return this.visitNumberNode(node, context);
    } else if (node instanceof UnaryOpNode) {
      return this.visitUnaryOpNode(node, context);
    } else if (node instanceof VarAcessNode) {
      return this.visitVarAccessNode(node, context);
    } else if (node instanceof VarAssignNode) {
      return this.visitVarAssignNode(node, context);
    } else if (node instanceof IfNode) {
      return this.visitIfNode(node, context);
    } else if (node instanceof ForNode) {
      return this.visitForNode(node, context);
    } else if (node instanceof WhileNode) {
      return this.visitWhileNode(node, context);
    } else if (node instanceof FuncDefNode) {
      return this.visitFuncDefNode(node, context);
    } else if (node instanceof CallNode) {
      return this.visitCallNode(node, context);
    } else {
      return this.noVisitMethod() as unknown as RuntimeResult;
    }
  }

  public noVisitMethod() {
    throw "No visit method found for node type\n";
  }

  public visitCallNode(node: CallNode, context: Context): RuntimeResult {
    const res = new RuntimeResult();
    const args = [];

    let valueToCall = res.register(this.visit(node.nodeToCall, context));
    if (res.error) return res;
    valueToCall = valueToCall?.clone().setPosition(
      node.positionStart,
      node.positionEnd,
    );

    for (const arg of node.argNodes) {
      args.push(res.register(this.visit(arg, context))!);
      if (res.error) return res;
    }

    const value = res.register(valueToCall?.execute(args));
    if (res.error) return res;

    return res.success(value!);
  }

  public visitFuncDefNode(node: FuncDefNode, context: Context): RuntimeResult {
    const res = new RuntimeResult();

    const name = node.varName?.value;
    const body = node.bodyNode;
    const args = node.argNameTokens;

    const value = new Function(name as string, body, args).setContext(context)
      .setPosition(node.positionStart, node.positionEnd);
    if (node.varName) {
      context.symbolTable?.set(
        name as string,
        new Variable(value, value.type, false),
      );
    }

    return res.success(value);
  }

  public visitForNode(
    node: ForNode,
    context: Context,
  ): RuntimeResult {
    const res = new RuntimeResult();

    const start = res.register(this.visit(node.startValue, context));
    if (res.error) return res;

    const end = res.register(this.visit(node.endValue, context));
    if (res.error) return res;

    let stepValue: BaseType;
    if (node.stepValueNode) {
      stepValue = res.register(this.visit(node.stepValueNode, context))!;
    } else {
      stepValue = new MyNumber(1);
    }

    let i = start?.value!;
    let condition;
    if (stepValue.value >= 0) {
      condition = i < end!.value;
    } else {
      condition = i > end!.value;
    }

    while (condition) {
      if (i == end?.value) break;
      context.symbolTable?.set(
        node.varNameToken.value as string,
        new Variable<Number>(i, start!.type, true),
      );
      i += stepValue.value!;
      res.register(this.visit(node.bodyNode, context));
      if (res.error) return res;
    }

    return res.success(null as unknown as MyNumber);
  }

  public visitWhileNode(
    node: WhileNode,
    context: Context,
  ): RuntimeResult {
    const res = new RuntimeResult();

    while (true) {
      const condition = res.register(this.visit(node.conditionNode, context));
      if (res.error) return res;

      if (!(condition?.value == 0 ? false : true)) break;

      res.register(this.visit(node.bodyNode, context));
      if (res.error) return res;
    }

    return res.success(null as any);
  }

  public visitIfNode(
    node: IfNode,
    context: Context,
  ): RuntimeResult {
    const res = new RuntimeResult();
    for (const [condition, expression] of node.cases) {
      const conditionValue = res.register(this.visit(condition, context));
      if (res.error) return res;

      if (conditionValue?.value === 1) {
        const exprValue = res.register(this.visit(expression, context));
        if (res.error) return res;
        return res.success(exprValue!);
      }
    }

    if (node.elseCase) {
      const elseValue = res.register(this.visit(node.elseCase, context));
      if (res.error) return res;
      return res.success(elseValue!);
    }

    return res;
  }

  public visitVarAccessNode(
    node: VarAcessNode,
    context: Context,
  ): RuntimeResult {
    const res = new RuntimeResult();
    const varName = node.token.value;
    let varValue = context.symbolTable?.get(varName as string).value;
    if (!varValue) {
      return res.failure(
        new RuntimeError(
          node.positionStart,
          node.positionEnd,
          `${varName} is not defined yet!`,
          context,
        ),
      );
    }
    varValue = varValue.clone().setPosition(
      node.positionStart,
      node.positionEnd,
    );
    return res.success(varValue!);
  }

  public visitVarAssignNode(
    node: VarAssignNode,
    context: Context,
  ): RuntimeResult {
    const res = new RuntimeResult();
    const varName = node.name.value;
    const varValue = res.register(this.visit(node.value, context));
    if (res.error) {
      return res;
    }

    if (node.type !== varValue?.type) {
      return res.failure(
        new RuntimeError(
          node.positionStart,
          node.positionEnd,
          `${varValue?.type} is not assignable to the type of ${node.type}`,
          context,
        ),
      );
    }

    const get = context.symbolTable?.get(varName as string);
    if (get && get.reassignable) {
      return res.failure(
        new RuntimeError(
          node.positionStart,
          node.positionEnd,
          "Cannot Reassign a constant",
          context,
        ),
      );
    }
    context.symbolTable?.set(
      varName as string,
      new Variable(varValue!, node.type, node.reassignable),
    );
    return res.success(varValue!);
  }

  public visitBinOpNode(node: BinOpNode, context: Context) {
    const res = new RuntimeResult();
    const left = res.register(
      this.visit(node.leftNode, context),
    )!;
    if (res.error) return res;
    const right = res.register(
      this.visit(node.rightNode, context),
    )!;
    if (res.error) return res;
    let final!: MyNumber;
    let err!: Err;

    if (node.opToken.type === PLUS) {
      const { result, error } = left.addTo(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.type === MINUS) {
      const { result, error } = left.subBy(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.type === MULTIPLY) {
      const { result, error } = left.multiBy(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.type === DIVIDE) {
      const { result, error } = left.divBy(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.type === POWER) {
      const { result, error } = left.powBy(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.type === DOUBLE_EQUALS) {
      const { result, error } = left.equals(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.type === NOT_EQUALS) {
      const { result, error } = left.notEquals(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.type === LESS_THAN) {
      const { result, error } = left.lessThan(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.type === LESS_THAN_EQUALS) {
      const { result, error } = left.lessThanEquals(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.type === GREATER_THAN) {
      const { result, error } = left.greaterThan(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.type === GREATER_THAN_EQUALS) {
      const { result, error } = left.greaterThanEquals(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.match(KEYWORD, "and")) {
      const { result, error } = left.and(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.match(KEYWORD, "or")) {
      const { result, error } = left.or(right)!;
      if (error) err = error;
      else if (result) final = result;
    }

    if (err) return res.failure(err);

    return res.success(final.setPosition(node.positionStart, node.positionEnd));
  }

  public visitNumberNode(node: NumberNode, context: Context) {
    const res = new RuntimeResult();
    return res.success(
      new MyNumber(Number(node.token.value!)).setPosition(
        node.positionStart,
        node.positionEnd,
      ).setContext(context),
    );
  }

  public visitUnaryOpNode(node: UnaryOpNode, context: Context) {
    const res = new RuntimeResult();
    let number = res.register(
      this.visit(node.node, context) as unknown as MyNumber,
    )!;
    let err!: Err;
    if (res.error) return res;

    if (node.opToken.type === MINUS) {
      const { result, error } = number.multiBy(new MyNumber(-1))!;
      if (error) err = error;
      else if (result) number = result;
    }
    if (node.opToken.match(KEYWORD, "not")) {
      const { result, error } = number.not()!;
      if (error) err = error;
      else if (result) number = result;
    }

    if (err) return res.failure(err);

    return res.success(
      number.setPosition(node.positionStart, node.positionEnd),
    );
  }
}
