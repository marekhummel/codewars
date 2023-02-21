# https://www.codewars.com/kata/53005a7b26d12be55c000243

import re
import operator


class ExpressionError(Exception):
    pass


class Interpreter:
    def __init__(self) -> None:
        self._vars = {}
        self._precedences = {"*": 2, "/": 2, "%": 2, "+": 1, "-": 1, "=": 0}
        self._operations = {
            "+": operator.add,
            "-": operator.sub,
            "*": operator.mul,
            "/": operator.truediv,
            "%": operator.mod,
        }

    def input(self, expression: str) -> float | str:
        tokens = self._tokenize(expression)
        if not tokens:
            return ""

        postfix = self._parse(tokens)

        stack = []
        for token in postfix:
            if isinstance(token, str) and token in "+-*/%":
                rhs = self._get_operand_value(stack.pop())
                lhs = self._get_operand_value(stack.pop())
                if lhs is None or rhs is None:
                    raise ExpressionError("Can't get operands")

                stack.append(self._operations[token](lhs, rhs))
            elif isinstance(token, str) and token == "=":
                rhs = self._get_operand_value(stack.pop())
                lhs = stack.pop()
                if not isinstance(lhs, str):
                    raise ExpressionError("Left side of assignment not var")
                self._vars[lhs] = rhs
                stack.append(rhs)
            else:
                stack.append(token)

        if len(stack) == 1 and (val := self._get_operand_value(stack[0])):
            return val
        raise ExpressionError("Can't reduce expression")

    def _tokenize(self, expression: str) -> list[str]:
        if not expression:
            return []

        regex = re.compile(
            r"\s*(=>|[-+*\/\%=\(\)]|[A-Za-z_][A-Za-z0-9_]*|[0-9]*\.?[0-9]+)\s*"
        )
        tokens = regex.findall(expression)
        return [s for s in tokens if not s.isspace()]

    def _parse(self, tokens: list[str]) -> list[str | float]:
        """Shunting Yard Algorithm"""
        stack = []
        output = []

        for token in tokens:
            if re.match(r"[0-9]*\.?[0-9]+", token):
                output.append(float(token))
            elif re.match(r"[A-Za-z_][A-Za-z0-9_]*", token):
                output.append(token)
            elif re.match(r"[-+*\/\%=]", token):
                prec = self._precedences[token]
                is_left_assoc = token != "="
                while stack:
                    stack_token = stack.pop()
                    if re.match(r"[-+*\/\%=]", stack_token):
                        stack_token_prec = self._precedences[stack_token]
                        if is_left_assoc and prec <= stack_token_prec:
                            output.append(stack_token)
                            continue

                    stack.append(stack_token)
                    break

                stack.append(token)
            elif token == "(":
                stack.append(token)
            elif token == ")":
                while True:
                    if not stack:
                        raise ExpressionError("Missing LParen")

                    stack_token = stack.pop()
                    if stack_token == "(":
                        break

                    output.append(stack_token)

        while stack:
            stack_token = stack.pop()
            if stack_token == "(":
                raise ExpressionError("Missing RParen")

            output.append(stack_token)

        return output

    def _get_operand_value(self, op: str) -> float | None:
        if isinstance(op, str):
            return self._vars[op] if op in self._vars else None
        elif isinstance(op, float):
            return op

        return None


# --------------------------------------------
expressions = [
    "1 + 1",
    "2 - 1",
    "2 * 3",
    "8 / 4",
    "7 % 4",
    "x = 1",
    "x",
    "x + 3",
]

interpreter = Interpreter()
for expr in expressions:
    print(f"{expr} -> {interpreter.input(expr)}")
print(f"\n{interpreter._vars}")
