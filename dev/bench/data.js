window.BENCHMARK_DATA = {
  "lastUpdate": 1764770858431,
  "repoUrl": "https://github.com/konard/uselessgoddess-dunes",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "68294279+uselessgoddess@users.noreply.github.com",
            "name": "uselessgoddess",
            "username": "uselessgoddess"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b9d638a0a06fdbb1367d01f63e8887c11586c47a",
          "message": "Merge pull request #13 from konard/issue-9-2fa720aa7697\n\nFix git identity configuration for gh-pages branch creation",
          "timestamp": "2025-12-02T18:24:33+03:00",
          "tree_id": "f24763708b848d59d63f0fcbe31f03b7cf504b6f",
          "url": "https://github.com/konard/uselessgoddess-dunes/commit/b9d638a0a06fdbb1367d01f63e8887c11586c47a"
        },
        "date": 1764689410269,
        "tool": "cargo",
        "benches": [
          {
            "name": "sbt_insert_100/usize/100",
            "value": 1822,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_100/nonzero/100",
            "value": 1376,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_1000/usize/1000",
            "value": 24360,
            "range": "± 531",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_1000/nonzero/1000",
            "value": 21815,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_10000/usize/10000",
            "value": 311915,
            "range": "± 752",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_10000/nonzero/10000",
            "value": 267900,
            "range": "± 778",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_search_100/usize/100",
            "value": 2538,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_search_1000/usize/1000",
            "value": 35512,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_search_10000/usize/10000",
            "value": 739175,
            "range": "± 2447",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_full_cycle_100/usize/100",
            "value": 2064,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_full_cycle_1000/usize/1000",
            "value": 25882,
            "range": "± 118",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "68294279+uselessgoddess@users.noreply.github.com",
            "name": "uselessgoddess",
            "username": "uselessgoddess"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0f4f09670fb1d45469d3ea91624fb42571eb37ad",
          "message": "Merge pull request #11 from konard/issue-10-9a1c12f6ea06\n\nRefactor trees crate: reorganize test utilities and add generic benchmarks",
          "timestamp": "2025-12-03T16:53:50+03:00",
          "tree_id": "aa4c174374fbca3c79aa902c0ec11a3a7de072b2",
          "url": "https://github.com/konard/uselessgoddess-dunes/commit/0f4f09670fb1d45469d3ea91624fb42571eb37ad"
        },
        "date": 1764770857820,
        "tool": "cargo",
        "benches": [
          {
            "name": "sbt_full_cycle_100",
            "value": 1567.93,
            "range": "± 13.22",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_full_cycle_1000",
            "value": 22035.1,
            "range": "± 1800.45",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_100",
            "value": 1444.12,
            "range": "± 26.72",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_1000",
            "value": 21659.27,
            "range": "± 194.98",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_10000",
            "value": 294660.45,
            "range": "± 3237.94",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_search_100",
            "value": 2059.49,
            "range": "± 28.79",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_search_1000",
            "value": 49367.93,
            "range": "± 604.23",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_search_10000",
            "value": 765559,
            "range": "± 12654.38",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}