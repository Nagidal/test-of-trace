Feature: Test if trace was emitted

    Scenario: Emit and check trace
        When I call the add function
        Then a trace is emitted
