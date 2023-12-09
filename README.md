# Analytics Platform

## Data Model

| Entity       | PK               | SK               | GSI1PK          | GSI1SK                | GSI2PK | GSI2SK |
| ------------ | ---------------- | ---------------- | --------------- | --------------------- | ------ | ------ |
| User         | USER#{id}        | USER#{id}        | EMAIL#{email}   | EMAIL#{email}         |        |        |
| Email        | EMAIL#{email}    | EMAIL#{email}    | USER#{id}       | USER#{id}             |        |        |
| Auth Session | AUTHSESSION#{id} | AUTHSESSION#{id} | USER#{id}       | USER#{id}             |        |        |
| Session      | SESSION#{id}     | SESSION#{id}     | USER#{id}       | USER#{id}             |        |        |
| Team         | TEAM#{name}      | TEAM#{name}      |                 |                       |        |        |
| Team Member  | TEAM#{name}      | USER#{id}        | USER#{id}       | TEAM#{name}           |        |        |
| Tool         | USER#{id}        | TOOL#{id}        | TOOLTYPE#{type} | TOOLVERSION#{version} |        |        |

