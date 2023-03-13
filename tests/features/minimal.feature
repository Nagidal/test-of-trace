Feature: Test if trace was emitted

    Scenario: Emit and catch trace
        When I call the add function
        Then a trace is emitted
