import { DIVIDE, MINUS, MULTIPLY, PLUS } from "../utils/constants.ts";
import { BinOpNode } from "./node/binary_op_node.ts";
import { NumberNode } from "./node/number_nodes.ts";
import { UnaryOpNode } from "./node/unary_op_node.ts";
import { Number as MyNumber } from "./number.ts";

export class Interpreter {
    public visit(node: BinOpNode | NumberNode | UnaryOpNode): MyNumber {
        let visitNode: string;
        if(node instanceof BinOpNode) {
            visitNode = "visitBinOpNode";
        } else if(node instanceof NumberNode) {
            visitNode = "visitNumberNode";
        } else {
            visitNode = "visitUnaryOpNode"
        }

        const method = this[visitNode as keyof Interpreter] ?? this.noVisitMethod
        return this[method.name as keyof Interpreter](node as any) as MyNumber;
    }

    public noVisitMethod(_node: UnaryOpNode | BinOpNode | NumberNode) {
        throw "No visit method found"
    }

    public visitBinOpNode(node: BinOpNode) {
        const left: MyNumber = this.visit(node.leftNode) as MyNumber;
        const right: MyNumber = this.visit(node.rightNode) as MyNumber;
        let result: MyNumber;

        if(node.opToken.type === PLUS) {
            result = left.addTo(right)!;
        } else if(node.opToken.type === MINUS) {
            result = left.subBy(right)!;
        } else if(node.opToken.type === MULTIPLY) {
            result = left.multiBy(right)!;
        } else if(node.opToken.type === DIVIDE) {
            result = left.divBy(right)!;
        }

        return result!.setPosition(node.positionStart, node.positionEnd);
    }

    public visitNumberNode(node: NumberNode) {
        return new MyNumber(Number(node.token.value!)).setPosition(node.positionStart, node.positionEnd);
    }

    public visitUnaryOpNode(node: UnaryOpNode) {
        let number = this.visit(node.node) as MyNumber;

        if(node.opToken.type === MINUS) {
            number = number.multiBy(new MyNumber(-1))!
        }

        return number.setPosition(node.positionStart, node.positionEnd);
    }
}