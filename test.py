import asyncio
from dataclasses import dataclass

"""
    Update Test Report

    Before Running this file, make sure you wait for the `cargo t > test_results / cargo test > test_results` command to finish
"""


@dataclass
class TestData:
    models: list[str]
    tests: list[str]
    results: list[str]
    notes: list[str]

    @staticmethod
    def empty() -> "TestData":
        return TestData([], [], [], [])

    def zipped(self) -> list[tuple[str, str, str]]:
        return list(zip(self.models, self.tests, self.results))


@dataclass
class Table:
    titles: list[str]
    contents: TestData


def parse_test_data(lines: list[str]) -> TestData:
    models: list[str] = []
    tests: list[str] = []
    results: list[str] = []
    for line in lines:
        items: list[str] = line.replace("test ", "", 1).split("::")
        models.append(items[0])
        tests.append(items[3].split(" ... ")[0])
        results.append(items[-1].split("...")[-1].replace("\n", "").replace(" ", ""))

    return TestData(models=models, tests=tests, results=results, notes=[])


def create_table(data: TestData) -> Table:
    titles: list[str] = ["Model", "Test", "Result", "Note"]
    return Table(titles, data)


def write_data_to_report(table: Table) -> None:
    with open("testreport.md", "w") as f:
        f.write("# Test Report\n")
        f.write("\n|")
        f.write("|".join(table.titles))
        f.write("|\n|")
        f.write("|".join("-" for _ in table.titles))
        f.write("|\n")
        for m, t, r in table.contents.zipped():
            f.write(f"|{m}|{t}|{r}||\n")


def fetch_data(path: str) -> list[str]:
    with open(path, "r") as f:
        return f.readlines()[2:-4]


def main():
    data: TestData = parse_test_data(fetch_data("test_results"))
    table: Table = create_table(data)
    write_data_to_report(table)


if __name__ == "__main__":
    main()
