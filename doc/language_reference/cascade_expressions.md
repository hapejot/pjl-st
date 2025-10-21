# Cascade Expressions

Cascade Expressions
receiver
    unaryMessage;
    + 23;
    at: 23 put: value;
    yourself
messages in a cascade are separated by ; each message is sent to  in sequence.semicolons receiver
Intermediate answers are ignored, but side-effects on  will be retained.receiver
The cascade answers the result of sending the last message to  (after sending all the preceding ones!)receiver