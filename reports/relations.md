# The rules, as relations

**Generated. Do not edit.** One table per relation in `spec/data/`, which is where the
rules are stated. `reports/state.md` is the same view over the state the scenario
leaves; this is the same view over the rules it played by.

**The relation is the word each row opens with, and a file holds as many as it likes** -
`rules.4x` opens thirteen of these and `schema.4x` the rest.

26 relations, 569 rows.

## rule

From `spec/data/rules.4x`. 15 row(s), 2 column(s).

| id  | name             |
| --- | ---------------- |
| 1   | move             |
| 2   | build-extractor  |
| 3   | work             |
| 4   | refresh          |
| 5   | end-turn         |
| 6   | build-bin        |
| 7   | discard-disorder |
| 8   | upkeep           |
| 9   | perish           |
| 10  | breed            |
| 11  | toil             |
| 12  | gather           |
| 13  | launch           |
| 14  | deploy           |
| 15  | build-pioneer    |

## input

From `spec/data/rules.4x`. 20 row(s), 5 column(s).

| id  | name  | of       | rule            | seq |
| --- | ----- | -------- | --------------- | --- |
| 1   | what  | unit     | move            | 1   |
| 2   | from  | place    | move            | 2   |
| 3   | to    | place    | move            | 3   |
| 4   | where | place    | build-extractor | 1   |
| 7   | what  | resource | build-extractor | 2   |
| 8   | where | place    | work            | 1   |
| 9   | what  | resource | work            | 2   |
| 11  | what  | relation | refresh         | 1   |
| 12  | trait | trait    | refresh         | 2   |
| 13  | where | place    | build-bin       | 1   |
| 14  | what  | resource | build-bin       | 2   |
| 15  | where | place    | upkeep          | 1   |
| 16  | where | place    | perish          | 1   |
| 17  | where | place    | breed           | 1   |
| 18  | where | place    | toil            | 1   |
| 19  | where | place    | gather          | 1   |
| 20  | where | place    | launch          | 1   |
| 21  | where | place    | deploy          | 1   |
| 22  | what  | founder  | deploy          | 2   |
| 23  | where | place    | build-pioneer   | 1   |

## clause

From `spec/data/rules.4x`. 50 row(s), 6 column(s).

| id  | name      | relation  | role    | rule             | seq |
| --- | --------- | --------- | ------- | ---------------- | --- |
| 29  | clause-29 | place     | require | move             | 1   |
| 30  | clause-30 | place     | require | move             | 2   |
| 2   | clause-2  | adjacency | require | move             | 3   |
| 3   | clause-3  | unit      | remove  | move             | 4   |
| 4   | clause-4  | unit      | add     | move             | 5   |
| 5   | clause-5  | labor     | remove  | build-extractor  | 1   |
| 6   | clause-6  | metal     | remove  | build-extractor  | 2   |
| 7   | clause-7  | extractor | add     | build-extractor  | 3   |
| 10  | clause-10 | deposit   | require | work             | 1   |
| 13  | clause-13 | extractor | remove  | work             | 2   |
| 11  | clause-11 | labor     | remove  | work             | 3   |
| 14  | clause-14 | extractor | add     | work             | 4   |
| 12  | clause-12 | resource  | add     | work             | 5   |
| 15  | clause-15 | unit      | put     | refresh          | 1   |
| 17  | clause-17 | labor     | remove  | build-bin        | 1   |
| 18  | clause-18 | metal     | remove  | build-bin        | 2   |
| 19  | clause-19 | bin       | add     | build-bin        | 3   |
| 20  | clause-20 | stock     | keep    | discard-disorder | 1   |
| 28  | clause-28 | citizen   | require | upkeep           | 1   |
| 21  | clause-21 | citizen   | remove  | upkeep           | 2   |
| 22  | clause-22 | food      | remove  | upkeep           | 3   |
| 23  | clause-23 | citizen   | add     | upkeep           | 4   |
| 24  | clause-24 | citizen   | remove  | perish           | 1   |
| 25  | clause-25 | citizen   | remove  | breed            | 1   |
| 26  | clause-26 | food      | remove  | breed            | 2   |
| 27  | clause-27 | citizen   | add     | breed            | 3   |
| 31  | clause-31 | citizen   | require | toil             | 1   |
| 32  | clause-32 | citizen   | remove  | toil             | 2   |
| 33  | clause-33 | citizen   | add     | toil             | 3   |
| 34  | clause-34 | labor     | add     | toil             | 4   |
| 39  | clause-39 | deposit   | require | gather           | 1   |
| 35  | clause-35 | ark       | require | gather           | 2   |
| 36  | clause-36 | ark       | remove  | gather           | 3   |
| 37  | clause-37 | ark       | add     | gather           | 4   |
| 38  | clause-38 | energy    | add     | gather           | 5   |
| 40  | clause-40 | place     | require | launch           | 1   |
| 41  | clause-41 | place     | require | launch           | 2   |
| 42  | clause-42 | labor     | remove  | launch           | 3   |
| 43  | clause-43 | metal     | remove  | launch           | 4   |
| 44  | clause-44 | energy    | remove  | launch           | 5   |
| 45  | clause-45 | ark       | add     | launch           | 6   |
| 46  | clause-46 | place     | require | deploy           | 1   |
| 47  | clause-47 | place     | require | deploy           | 2   |
| 48  | clause-48 | founder   | remove  | deploy           | 3   |
| 49  | clause-49 | extractor | add     | deploy           | 4   |
| 50  | clause-50 | extractor | add     | deploy           | 5   |
| 51  | clause-51 | citizen   | add     | deploy           | 6   |
| 52  | clause-52 | labor     | remove  | build-pioneer    | 1   |
| 53  | clause-53 | metal     | remove  | build-pioneer    | 2   |
| 54  | clause-54 | pioneer   | add     | build-pioneer    | 3   |

## relation-of

From `spec/data/rules.4x`. 5 row(s), 2 column(s).

| clause    | input |
| --------- | ----- |
| clause-3  | what  |
| clause-4  | what  |
| clause-12 | what  |
| clause-15 | what  |
| clause-48 | what  |

## binding

From `spec/data/rules.4x`. 46 row(s), 4 column(s).

| clause    | column | id  | input |
| --------- | ------ | --- | ----- |
| clause-29 | 134    | 42  | from  |
| clause-30 | 134    | 43  | to    |
| clause-3  | 72     | 5   | from  |
| clause-4  | 72     | 7   | to    |
| clause-5  | 78     | 9   | where |
| clause-6  | 80     | 10  | where |
| clause-7  | 55     | 11  | where |
| clause-7  | 56     | 12  | what  |
| clause-10 | 51     | 17  | where |
| clause-10 | 52     | 18  | what  |
| clause-13 | 55     | 26  | where |
| clause-13 | 56     | 27  | what  |
| clause-11 | 78     | 19  | where |
| clause-14 | 55     | 28  | where |
| clause-14 | 56     | 29  | what  |
| clause-12 | 74     | 21  | where |
| clause-17 | 78     | 30  | where |
| clause-18 | 80     | 31  | where |
| clause-19 | 118    | 32  | where |
| clause-19 | 119    | 33  | what  |
| clause-28 | 130    | 41  | where |
| clause-21 | 130    | 34  | where |
| clause-22 | 82     | 35  | where |
| clause-23 | 130    | 36  | where |
| clause-24 | 130    | 37  | where |
| clause-25 | 130    | 38  | where |
| clause-26 | 82     | 39  | where |
| clause-27 | 130    | 40  | where |
| clause-31 | 130    | 44  | where |
| clause-32 | 130    | 45  | where |
| clause-33 | 130    | 46  | where |
| clause-34 | 78     | 47  | where |
| clause-39 | 51     | 52  | where |
| clause-35 | 142    | 48  | where |
| clause-36 | 142    | 49  | where |
| clause-37 | 142    | 50  | where |
| clause-38 | 140    | 51  | where |
| clause-40 | 134    | 53  | where |
| clause-42 | 78     | 54  | where |
| clause-43 | 80     | 55  | where |
| clause-44 | 140    | 56  | where |
| clause-46 | 134    | 57  | where |
| clause-48 | 153    | 58  | where |
| clause-52 | 78     | 59  | where |
| clause-53 | 80     | 60  | where |
| clause-54 | 150    | 61  | where |

## reading

From `spec/data/rules.4x`. 16 row(s), 5 column(s).

| clause    | column | id  | of        | takes |
| --------- | ------ | --- | --------- | ----- |
| clause-2  | 42     | 3   | clause-29 | 135   |
| clause-2  | 43     | 4   | clause-30 | 135   |
| clause-30 | 136    | 10  | clause-29 | 136   |
| clause-12 | 75     | 1   | clause-10 | 53    |
| clause-23 | 133    | 2   | clause-28 | 133   |
| clause-23 | 137    | 7   | clause-28 | 137   |
| clause-33 | 131    | 5   | clause-31 | 131   |
| clause-33 | 133    | 6   | clause-31 | 133   |
| clause-37 | 143    | 8   | clause-35 | 143   |
| clause-38 | 141    | 9   | clause-39 | 53    |
| clause-41 | 135    | 11  | clause-40 | 135   |
| clause-45 | 142    | 12  | clause-41 | 134   |
| clause-47 | 135    | 13  | clause-46 | 135   |
| clause-49 | 55     | 14  | clause-47 | 134   |
| clause-50 | 55     | 15  | clause-47 | 134   |
| clause-51 | 130    | 16  | clause-47 | 134   |

## literal

From `spec/data/rules.4x`. 67 row(s), 4 column(s).

| clause    | column | id  | value   |
| --------- | ------ | --- | ------- |
| clause-3  | 98     | 13  | 1       |
| clause-3  | 73     | 2   | 1       |
| clause-4  | 98     | 14  | 0       |
| clause-4  | 73     | 1   | 1       |
| clause-5  | 79     | 4   | 1       |
| clause-6  | 81     | 6   | 1       |
| clause-7  | 101    | 15  | 1       |
| clause-7  | 57     | 8   | 1       |
| clause-13 | 101    | 16  | 1       |
| clause-13 | 57     | 17  | 1       |
| clause-11 | 79     | 11  | 1       |
| clause-14 | 101    | 18  | 0       |
| clause-14 | 57     | 19  | 1       |
| clause-17 | 79     | 20  | 1       |
| clause-18 | 81     | 21  | 1       |
| clause-19 | 120    | 22  | 1       |
| clause-28 | 131    | 36  | 1       |
| clause-21 | 131    | 23  | 1       |
| clause-21 | 132    | 24  | 1       |
| clause-22 | 83     | 25  | 1       |
| clause-23 | 131    | 26  | 0       |
| clause-23 | 132    | 27  | 1       |
| clause-24 | 131    | 28  | 1       |
| clause-25 | 131    | 29  | 0       |
| clause-25 | 133    | 30  | 1       |
| clause-25 | 132    | 31  | 1       |
| clause-26 | 83     | 32  | 1       |
| clause-27 | 131    | 33  | 0       |
| clause-27 | 133    | 34  | 1       |
| clause-27 | 137    | 48  | 1       |
| clause-27 | 132    | 35  | 2       |
| clause-31 | 137    | 42  | 1       |
| clause-32 | 137    | 43  | 1       |
| clause-32 | 132    | 44  | 1       |
| clause-33 | 137    | 45  | 0       |
| clause-33 | 132    | 46  | 1       |
| clause-34 | 79     | 47  | 1       |
| clause-35 | 144    | 49  | 1       |
| clause-36 | 144    | 50  | 1       |
| clause-36 | 145    | 51  | 1       |
| clause-37 | 144    | 52  | 0       |
| clause-37 | 145    | 53  | 1       |
| clause-39 | 52     | 55  | energy  |
| clause-40 | 136    | 56  | surface |
| clause-41 | 136    | 57  | orbit   |
| clause-42 | 79     | 58  | 1       |
| clause-43 | 81     | 59  | 1       |
| clause-44 | 141    | 60  | 1       |
| clause-45 | 143    | 61  | 1       |
| clause-45 | 144    | 62  | 1       |
| clause-45 | 145    | 63  | 1       |
| clause-47 | 136    | 65  | surface |
| clause-48 | 154    | 66  | 1       |
| clause-49 | 56     | 67  | metal   |
| clause-49 | 101    | 68  | 1       |
| clause-49 | 57     | 69  | 1       |
| clause-50 | 56     | 70  | food    |
| clause-50 | 101    | 71  | 1       |
| clause-50 | 57     | 72  | 1       |
| clause-51 | 131    | 73  | 0       |
| clause-51 | 133    | 74  | 1       |
| clause-51 | 137    | 75  | 1       |
| clause-51 | 132    | 76  | 2       |
| clause-52 | 79     | 77  | 1       |
| clause-53 | 81     | 78  | 1       |
| clause-54 | 151    | 79  | 1       |
| clause-54 | 152    | 80  | 1       |

## assigns

From `spec/data/rules.4x`. 1 row(s), 4 column(s).

| clause    | id  | input | value |
| --------- | --- | ----- | ----- |
| clause-15 | 1   | trait | 1     |

## part

From `spec/data/rules.4x`. 10 row(s), 5 column(s).

| id  | is               | name    | of       | seq |
| --- | ---------------- | ------- | -------- | --- |
| 4   | upkeep           | part-4  | end-turn | 1   |
| 6   | perish           | part-6  | end-turn | 2   |
| 7   | breed            | part-7  | end-turn | 3   |
| 3   | discard-disorder | part-3  | end-turn | 4   |
| 1   | refresh          | part-1  | end-turn | 5   |
| 2   | refresh          | part-2  | end-turn | 6   |
| 5   | refresh          | part-5  | end-turn | 7   |
| 8   | refresh          | part-8  | end-turn | 8   |
| 9   | refresh          | part-9  | end-turn | 9   |
| 10  | refresh          | part-10 | end-turn | 10  |

## argument

From `spec/data/rules.4x`. 12 row(s), 4 column(s).

| id  | input | part    | value     |
| --- | ----- | ------- | --------- |
| 1   | what  | part-1  | unit      |
| 2   | trait | part-1  | moving    |
| 3   | what  | part-2  | extractor |
| 4   | trait | part-2  | working   |
| 5   | what  | part-5  | citizen   |
| 6   | trait | part-5  | hungry    |
| 7   | what  | part-8  | citizen   |
| 8   | trait | part-8  | bearing   |
| 9   | what  | part-9  | citizen   |
| 10  | trait | part-9  | laboring  |
| 11  | what  | part-10 | ark       |
| 12  | trait | part-10 | gathering |

## scope

From `spec/data/rules.4x`. 3 row(s), 2 column(s).

| input | rule   |
| ----- | ------ |
| where | upkeep |
| where | perish |
| where | breed  |

## repeats

From `spec/data/rules.4x`. 4 row(s), 1 column(s).

| rule   |
| ------ |
| upkeep |
| perish |
| breed  |
| toil   |

## soft

From `spec/data/rules.4x`. 2 row(s), 1 column(s).

| clause    |
| --------- |
| clause-49 |
| clause-50 |

## relation

From `spec/data/schema.4x`. 49 row(s), 2 column(s).

| id  | name        |
| --- | ----------- |
| 1   | relation    |
| 2   | column      |
| 3   | reference   |
| 4   | state       |
| 5   | role        |
| 6   | rule        |
| 7   | input       |
| 8   | clause      |
| 9   | binding     |
| 12  | primitive   |
| 13  | territory   |
| 15  | adjacency   |
| 17  | literal     |
| 18  | deposit     |
| 19  | extractor   |
| 21  | reading     |
| 22  | attribute   |
| 23  | relation-of |
| 24  | family      |
| 25  | member      |
| 26  | unit        |
| 27  | resource    |
| 28  | scout       |
| 29  | labor       |
| 30  | metal       |
| 31  | food        |
| 33  | supply      |
| 34  | provides    |
| 35  | transport   |
| 36  | consumes    |
| 37  | assigns     |
| 38  | trait       |
| 39  | carries     |
| 40  | part        |
| 41  | argument    |
| 42  | bin         |
| 43  | capacity    |
| 44  | loose       |
| 45  | repeats     |
| 46  | scope       |
| 53  | soft        |
| 47  | citizen     |
| 50  | energy      |
| 51  | ark         |
| 52  | stands-in   |
| 49  | stock       |
| 48  | place       |
| 54  | pioneer     |
| 55  | founder     |

## column

From `spec/data/schema.4x`. 138 row(s), 4 column(s).

| id  | name      | relation    | seq |
| --- | --------- | ----------- | --- |
| 1   | id        | relation    | 1   |
| 2   | name      | relation    | 2   |
| 3   | id        | column      | 1   |
| 4   | relation  | column      | 2   |
| 5   | seq       | column      | 3   |
| 6   | name      | column      | 4   |
| 7   | id        | reference   | 1   |
| 8   | column    | reference   | 2   |
| 9   | to        | reference   | 3   |
| 10  | id        | state       | 1   |
| 11  | relation  | state       | 2   |
| 12  | id        | role        | 1   |
| 13  | name      | role        | 2   |
| 14  | id        | rule        | 1   |
| 15  | name      | rule        | 2   |
| 16  | id        | input       | 1   |
| 17  | rule      | input       | 2   |
| 18  | seq       | input       | 3   |
| 19  | name      | input       | 4   |
| 20  | of        | input       | 5   |
| 21  | id        | clause      | 1   |
| 22  | rule      | clause      | 2   |
| 23  | seq       | clause      | 3   |
| 24  | role      | clause      | 4   |
| 25  | relation  | clause      | 5   |
| 26  | id        | binding     | 1   |
| 27  | clause    | binding     | 2   |
| 28  | column    | binding     | 3   |
| 29  | input     | binding     | 4   |
| 36  | id        | primitive   | 1   |
| 37  | word      | primitive   | 2   |
| 38  | id        | territory   | 1   |
| 41  | id        | adjacency   | 1   |
| 42  | from      | adjacency   | 2   |
| 43  | to        | adjacency   | 3   |
| 47  | id        | literal     | 1   |
| 48  | clause    | literal     | 2   |
| 49  | column    | literal     | 3   |
| 50  | value     | literal     | 4   |
| 51  | where     | deposit     | 1   |
| 52  | what      | deposit     | 2   |
| 53  | density   | deposit     | 3   |
| 54  | quantity  | deposit     | 4   |
| 55  | where     | extractor   | 1   |
| 56  | what      | extractor   | 2   |
| 101 | working   | extractor   | 3   |
| 57  | quantity  | extractor   | 4   |
| 60  | id        | reading     | 1   |
| 61  | clause    | reading     | 2   |
| 62  | column    | reading     | 3   |
| 63  | of        | reading     | 4   |
| 64  | takes     | reading     | 5   |
| 65  | column    | attribute   | 1   |
| 66  | relation  | attribute   | 2   |
| 67  | clause    | relation-of | 1   |
| 68  | input     | relation-of | 2   |
| 69  | relation  | family      | 1   |
| 70  | kind      | member      | 1   |
| 71  | family    | member      | 2   |
| 72  | where     | unit        | 1   |
| 98  | moving    | unit        | 2   |
| 73  | quantity  | unit        | 3   |
| 74  | where     | resource    | 1   |
| 75  | quantity  | resource    | 2   |
| 76  | where     | scout       | 1   |
| 99  | moving    | scout       | 2   |
| 77  | quantity  | scout       | 3   |
| 78  | where     | labor       | 1   |
| 79  | quantity  | labor       | 2   |
| 80  | where     | metal       | 1   |
| 81  | quantity  | metal       | 2   |
| 82  | where     | food        | 1   |
| 83  | quantity  | food        | 2   |
| 102 | id        | assigns     | 1   |
| 103 | clause    | assigns     | 2   |
| 104 | input     | assigns     | 3   |
| 105 | value     | assigns     | 4   |
| 106 | id        | trait       | 1   |
| 107 | name      | trait       | 2   |
| 108 | kind      | carries     | 1   |
| 109 | trait     | carries     | 2   |
| 110 | id        | part        | 1   |
| 111 | of        | part        | 2   |
| 112 | is        | part        | 3   |
| 113 | seq       | part        | 4   |
| 114 | id        | argument    | 1   |
| 115 | part      | argument    | 2   |
| 116 | input     | argument    | 3   |
| 117 | value     | argument    | 4   |
| 118 | where     | bin         | 1   |
| 119 | what      | bin         | 2   |
| 120 | quantity  | bin         | 3   |
| 121 | of        | capacity    | 1   |
| 122 | for       | capacity    | 2   |
| 123 | what      | capacity    | 3   |
| 124 | per       | capacity    | 4   |
| 125 | quantity  | capacity    | 5   |
| 126 | kind      | loose       | 1   |
| 87  | id        | supply      | 1   |
| 88  | name      | supply      | 2   |
| 89  | per       | supply      | 3   |
| 90  | kind      | provides    | 1   |
| 91  | what      | provides    | 2   |
| 92  | quantity  | provides    | 3   |
| 93  | kind      | consumes    | 1   |
| 94  | what      | consumes    | 2   |
| 95  | quantity  | consumes    | 3   |
| 96  | where     | transport   | 1   |
| 100 | moving    | transport   | 2   |
| 97  | quantity  | transport   | 3   |
| 127 | rule      | repeats     | 1   |
| 128 | rule      | scope       | 1   |
| 129 | input     | scope       | 2   |
| 149 | clause    | soft        | 1   |
| 130 | where     | citizen     | 1   |
| 131 | hungry    | citizen     | 2   |
| 133 | bearing   | citizen     | 3   |
| 137 | laboring  | citizen     | 4   |
| 132 | quantity  | citizen     | 5   |
| 140 | where     | energy      | 1   |
| 141 | quantity  | energy      | 2   |
| 142 | where     | ark         | 1   |
| 143 | moving    | ark         | 2   |
| 144 | gathering | ark         | 3   |
| 145 | quantity  | ark         | 4   |
| 146 | kind      | stands-in   | 1   |
| 147 | per       | stands-in   | 2   |
| 148 | layer     | stands-in   | 3   |
| 138 | where     | stock       | 1   |
| 139 | quantity  | stock       | 2   |
| 134 | id        | place       | 1   |
| 135 | of        | place       | 2   |
| 136 | layer     | place       | 3   |
| 150 | where     | pioneer     | 1   |
| 151 | moving    | pioneer     | 2   |
| 152 | quantity  | pioneer     | 3   |
| 153 | where     | founder     | 1   |
| 154 | quantity  | founder     | 2   |

## reference

From `spec/data/schema.4x`. 67 row(s), 3 column(s).

| column | id  | to        |
| ------ | --- | --------- |
| 4      | 1   | relation  |
| 8      | 2   | column    |
| 9      | 3   | relation  |
| 11     | 4   | relation  |
| 17     | 5   | rule      |
| 20     | 6   | relation  |
| 22     | 7   | rule      |
| 24     | 8   | role      |
| 25     | 9   | relation  |
| 27     | 10  | clause    |
| 28     | 11  | column    |
| 29     | 12  | input     |
| 42     | 16  | territory |
| 43     | 17  | territory |
| 48     | 20  | clause    |
| 49     | 21  | column    |
| 51     | 22  | place     |
| 52     | 23  | resource  |
| 55     | 24  | place     |
| 56     | 25  | resource  |
| 61     | 28  | clause    |
| 62     | 29  | column    |
| 63     | 30  | clause    |
| 64     | 31  | column    |
| 65     | 32  | column    |
| 66     | 33  | relation  |
| 67     | 34  | clause    |
| 68     | 35  | input     |
| 69     | 36  | relation  |
| 70     | 37  | relation  |
| 71     | 38  | relation  |
| 76     | 41  | place     |
| 78     | 42  | place     |
| 80     | 43  | place     |
| 82     | 44  | place     |
| 103    | 45  | clause    |
| 104    | 46  | input     |
| 89     | 47  | relation  |
| 90     | 48  | relation  |
| 91     | 49  | supply    |
| 93     | 50  | relation  |
| 94     | 51  | supply    |
| 96     | 52  | place     |
| 108    | 53  | relation  |
| 109    | 54  | trait     |
| 111    | 55  | rule      |
| 112    | 56  | rule      |
| 115    | 57  | part      |
| 116    | 58  | input     |
| 118    | 59  | place     |
| 119    | 60  | resource  |
| 121    | 61  | relation  |
| 122    | 62  | relation  |
| 123    | 63  | resource  |
| 124    | 64  | relation  |
| 126    | 65  | relation  |
| 127    | 66  | rule      |
| 128    | 67  | rule      |
| 129    | 68  | input     |
| 149    | 75  | clause    |
| 130    | 69  | place     |
| 140    | 71  | place     |
| 142    | 72  | place     |
| 146    | 73  | relation  |
| 147    | 74  | relation  |
| 135    | 70  | territory |
| 150    | 76  | place     |

## state

From `spec/data/schema.4x`. 18 row(s), 2 column(s).

| id  | relation  |
| --- | --------- |
| 1   | territory |
| 3   | adjacency |
| 5   | deposit   |
| 6   | extractor |
| 7   | scout     |
| 8   | labor     |
| 9   | metal     |
| 10  | food      |
| 12  | transport |
| 13  | provides  |
| 14  | consumes  |
| 15  | bin       |
| 16  | capacity  |
| 17  | citizen   |
| 19  | energy    |
| 20  | ark       |
| 18  | place     |
| 21  | pioneer   |

## family

From `spec/data/schema.4x`. 4 row(s), 1 column(s).

| relation |
| -------- |
| unit     |
| resource |
| stock    |
| founder  |

## member

From `spec/data/schema.4x`. 13 row(s), 2 column(s).

| family   | kind      |
| -------- | --------- |
| unit     | scout     |
| unit     | ark       |
| unit     | transport |
| resource | metal     |
| resource | food      |
| resource | energy    |
| stock    | metal     |
| stock    | food      |
| stock    | labor     |
| stock    | energy    |
| unit     | pioneer   |
| founder  | pioneer   |
| founder  | ark       |

## attribute

From `spec/data/schema.4x`. 3 row(s), 2 column(s).

| column | relation    |
| ------ | ----------- |
| 53     | deposit     |
| 66     | attribute   |
| 68     | relation-of |

## supply

From `spec/data/schema.4x`. 1 row(s), 3 column(s).

| id  | name  | per   |
| --- | ----- | ----- |
| 1   | berth | place |

## role

From `spec/data/schema.4x`. 5 row(s), 2 column(s).

| id  | name    |
| --- | ------- |
| 1   | require |
| 2   | remove  |
| 3   | add     |
| 4   | put     |
| 5   | keep    |

## trait

From `spec/data/schema.4x`. 6 row(s), 2 column(s).

| id  | name      |
| --- | --------- |
| 1   | moving    |
| 2   | working   |
| 3   | hungry    |
| 4   | bearing   |
| 5   | laboring  |
| 6   | gathering |

## carries

From `spec/data/schema.4x`. 10 row(s), 2 column(s).

| kind      | trait     |
| --------- | --------- |
| unit      | moving    |
| scout     | moving    |
| transport | moving    |
| extractor | working   |
| citizen   | hungry    |
| citizen   | bearing   |
| citizen   | laboring  |
| ark       | moving    |
| ark       | gathering |
| pioneer   | moving    |

## loose

From `spec/data/schema.4x`. 1 row(s), 1 column(s).

| kind  |
| ----- |
| stock |

## stands-in

From `spec/data/schema.4x`. 3 row(s), 3 column(s).

| kind      | layer   | per   |
| --------- | ------- | ----- |
| ark       | orbit   | place |
| extractor | surface | place |
| pioneer   | surface | place |

