# Uncommon Lisp

Uncommon Lisp основан на выражениях. Выражение это список в круглых скобках, первым элементом которого является функция.

## Datatypes

- Number
- String
- Bool
- Nil
- List
- Object

## Examples

```lisp
(var name "Hello")
(var age (add 10 10))
(var list [1 2 3])
(var person {name:"Alex" age:20})
(print (typeof person.name))

# Empty expression
() -> Nil

# while loop
(set count 10)
(while (not (eq count 0)) 
	(set count (sub count 1)) 
	(print count))

# Простая функция сложения двух чисел
(func addTwoNumbers [a b])
  (add a b))

(addTwoNumbers 5 3)

# Рекурсивная функция для вычисления факториала:
(func factorial [n]
  (if (le n 1)
      1
      (* n (factorial (sub n 1)))))

(factorial 5) # Вернет 120
```
