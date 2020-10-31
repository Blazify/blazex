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
import { IfNode } from "./node/if_node.ts";
import { NumberNode } from "./node/number_nodes.ts";
import { UnaryOpNode } from "./node/unary_op_node.ts";
import { VarAcessNode } from "./node/var_access_node.ts";
import { VarAssignNode } from "./node/var_assign_node.ts";
import { Number as MyNumber } from "./number.ts";
import { RuntimeResult } from "./runtime_result.ts";

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
    } else {
      return this.noVisitMethod(node, context) as unknown as RuntimeResult;
    }
  }

  public noVisitMethod(
    _node: Nodes,
    _context: Context,
  ) {
    throw "No visit method found for node type\n";
  }

  public visitIfNode(
    node: IfNode,
    context: Context,
  ): RuntimeResult {
    const res = new RuntimeResult();
   for(const [condition, expression] of node.cases) {
     const conditionValue = res.register(this.visit(condition, context));
     if(res.error) return res;

     if(conditionValue?.value === 1) {
       const exprValue = res.register(this.visit(expression, context));
       if(res.error) return res;
       return res.success(exprValue!);
     }
   }

   if(node.elseCase) {
     const elseValue = res.register(this.visit(node.elseCase, context));
     if(res.error) return res;
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
    let varValue = context.symbolTable?.get<MyNumber>(varName as string)?.value;
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
    return res.success(varValue);
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
    const get = context.symbolTable?.get(varName as string)
    if(get && !get.reassignable) return res.failure(new RuntimeError(node.positionStart, node.positionEnd, "Cannot Reassign a constant", context))
    context.symbolTable?.set(varName as string, new Variable<MyNumber>(varValue!, node.type, node.reassignable));
    return res.success(varValue!);
  }

  public visitBinOpNode(node: BinOpNode, context: Context) {
    const res = new RuntimeResult();
    const left: MyNumber = res.register(
      this.visit(node.leftNode, context) as unknown as MyNumber,
    )!;
    if (res.error) return res;
    const right: MyNumber = res.register(
      this.visit(node.rightNode, context) as unknown as MyNumber,
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
