# Anubhav Programming Language - Complete Feature List

## Overview
Anubhav is a comprehensive, intention-based programming language with 100+ operations and advanced features.

## Complete Feature Set

### 1. Core Language Features
- **INTENT/MANIFEST** - Intention-based programming paradigm
- **STORE/RECALL** - Variable storage and retrieval
- **CALCULATE** - Mathematical expressions
- **COMBINE** - String concatenation
- **PRINT** - Output with string interpolation

### 2. Data Types & Structures
- **Numbers** - Floating point arithmetic
- **Strings** - Text manipulation
- **Arrays** - Dynamic lists
- **Dictionaries** - Key-value pairs (HashMap)
- **Functions** - First-class functions with local scope

### 3. Control Flow
- **IF/THEN/ELSE** - Conditional execution
- **FOR** - Range-based loops with STEP
- **WHILE** - Condition-based loops
- **REPEAT** - Count-based loops
- **SWITCH/CASE/DEFAULT** - Pattern matching
- **BREAK/CONTINUE** - Loop control
- **TRY/CATCH** - Error handling
- **ASSERT** - Runtime assertions

### 4. Array Operations (20+ operations)
- **Basic**: ARRAY, PUSH, POP, GET, SET, SIZE
- **Transformations**: MAP, FILTER, REVERSE, SORT (ASC/DESC)
- **Aggregations**: SUM, COUNT, AVERAGE, MEDIAN, MODE
- **Advanced**: JOIN, UNIQUE, FLATTEN, ZIP, CONCAT
- **Slicing**: TAKE, DROP, SLICE
- **Statistical**: STDDEV, VARIANCE

### 5. String Operations (15+ operations)
- **Case**: UPPERCASE, LOWERCASE
- **Manipulation**: TRIM, PAD, REPLACE, SPLIT
- **Inspection**: LENGTH, STARTS_WITH, ENDS_WITH, INCLUDES, INDEX_OF
- **Substring**: SUBSTRING extraction

### 6. Mathematical Functions (15+ operations)
- **Basic**: +, -, *, /, %, ** (power)
- **Comparisons**: ==, !=, <, >, <=, >=
- **Logical**: AND, OR, NOT
- **Functions**: MIN, MAX, FLOOR, CEIL, ROUND
- **Advanced**: RANDOM, MIN_OF, MAX_OF
- **Aggregates**: AVERAGE, SUM

### 7. Functions & Modules
- **FUNCTION** - Define named functions
- **CALL** - Invoke functions with arguments
- **RETURN** - Return values from functions
- **LAMBDA** - Anonymous functions (planned)
- **IMPORT/EXPORT** - Module system
- **Recursion** - Full recursion support
- **Local Scope** - Call stack management

### 8. I/O Operations
- **Console**: PRINT, INPUT
- **Files**: READ_FILE, WRITE_FILE, APPEND_FILE, EXISTS
- **Formatting**: String interpolation with ${}

### 9. Dictionary Operations
- **DICT** - Create dictionary
- **PUT** - Set key-value pair
- **FETCH** - Get value by key
- **KEYS** - Get all keys
- **VALUES** - Get all values
- **DELETE** - Remove key
- **MERGE** - Combine dictionaries

### 10. Advanced Features (Planned/Partial)
- **PIPE** - Function composition
- **RANGE** - Generate number sequences
- **FOLD** - Reduce operations
- **LAMBDA** - Anonymous functions
- **EVAL** - Dynamic code evaluation
- **TYPE_OF** - Type checking
- **CLONE** - Deep copying
- **SHUFFLE** - Random array ordering
- **SAMPLE** - Random selection
- **SLEEP** - Delay execution
- **CLEAR** - Clear data structures
- **SWAP** - Exchange values

### 11. Utility Operations
- **INCREMENT/DECREMENT** - Variable modification
- **PARSE** - String to number conversion
- **TO_STRING** - Number to string conversion
- **TYPE** - Get variable type
- **EXISTS** - Check file existence

## Language Statistics
- **Total Operations**: 100+
- **Built-in Functions**: 60+
- **Data Structures**: 4 (Numbers, Strings, Arrays, Dictionaries)
- **Control Structures**: 8
- **Array Operations**: 25+
- **String Operations**: 15+
- **Math Operations**: 20+
- **I/O Operations**: 8+

## Example Programs

### Hello World
```anubhav
INTENT greeting "Hello, World!"
MANIFEST greeting
```

### Fibonacci Sequence
```anubhav
FUNCTION fibonacci(n) DO
    IF RECALL n <= 1 THEN
        RETURN RECALL n
    END
    CALCULATE n1 RECALL n - 1
    CALCULATE n2 RECALL n - 2
    CALL fibonacci(RECALL n1) fib1
    CALL fibonacci(RECALL n2) fib2
    CALCULATE result RECALL fib1 + RECALL fib2
    RETURN RECALL result
END

FOR i 0 TO 10 DO
    CALL fibonacci(RECALL i) result
    PRINT "fib(" i ") =" result
END
```

### Array Processing
```anubhav
ARRAY data
PUSH data 5
PUSH data 2
PUSH data 8
PUSH data 1
PUSH data 9

# Sort and process
SORT data ASC
FILTER data RECALL item > 5 large_nums
MAP large_nums RECALL item * 2 doubled
SUM doubled total
PRINT "Result:" total
```

## Language Philosophy
Anubhav follows an intention-based programming paradigm where:
- **INTENT** declares what you want to achieve
- **MANIFEST** makes it happen
- Clear, readable syntax for educational purposes
- Comprehensive feature set for real-world programming
- Extensible architecture for future enhancements

## Status
✅ **Production Ready Features**: 80+ operations fully implemented and tested
🚧 **In Development**: Advanced functional programming features
📋 **Planned**: Object-oriented programming, async/await, pattern matching

---
*Anubhav - A complete, modern programming language built from scratch*