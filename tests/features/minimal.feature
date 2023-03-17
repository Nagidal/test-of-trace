Feature: Test if trace was emitted

    Scenario Outline: Emit and check addition trace
        When I call the add function with <left> and <right>
        Then a trace about adding <left> and <right> is emitted
        And the result <result> is written in the <level> trace

        Examples:
            | left | right | result | level |
            | 1    | 2     | 3      | info  |
            | 3    | 5     | 8      | info  |
            | 7    | 11    | 18     | info  |
            | 13   | 17    | 30     | info  |
            | 14   | 17    | 31     | info  |
            | 15   | 17    | 32     | info  |
            | 16   | 17    | 33     | info  |
            | 17   | 17    | 34     | info  |
            | 18   | 17    | 35     | info  |
            | 19   | 17    | 36     | info  |
            | 20   | 17    | 37     | info  |
            | 21   | 17    | 38     | info  |
            | 22   | 17    | 39     | info  |
            | 23   | 17    | 40     | info  |
            | 24   | 17    | 41     | info  |
            | 25   | 17    | 42     | info  |
            | 26   | 17    | 43     | info  |

    Scenario Outline: Emit and check multiplication trace
        When I call the multiply function with <left> and <right>
        Then a trace about multiplying <left> and <right> is emitted
        And the result <result> is written in the <level> trace

        Examples:
            | left | right | result | level |
            | 1    | 2     | 2      | info  |
            | 3    | 5     | 15     | info  |
            | 7    | 11    | 77     | info  |
            | 13   | 17    | 221    | info  |
            | 14   | 17    | 238    | info  |
            | 15   | 17    | 255    | info  |
            | 16   | 17    | 272    | info  |
            | 17   | 17    | 289    | info  |
            | 18   | 17    | 306    | info  |
            | 19   | 17    | 323    | info  |
            | 20   | 17    | 340    | info  |
            | 21   | 17    | 357    | info  |
            | 22   | 17    | 374    | info  |
            | 23   | 17    | 391    | info  |
            | 24   | 17    | 408    | info  |
            | 25   | 17    | 425    | info  |
            | 26   | 17    | 442    | info  |
