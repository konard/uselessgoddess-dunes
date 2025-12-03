window.BENCHMARK_DATA = {
  "lastUpdate": 1764792580236,
  "repoUrl": "https://github.com/uselessgoddess/dunes",
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
          "id": "ea41f4eb0c86b9efe0cc74d78cbe487b40df3bc4",
          "message": "Merge pull request #25 from konard/issue-24-8b8684529f8d\n\nImprove benchmarks quality",
          "timestamp": "2025-12-03T23:08:51+03:00",
          "tree_id": "a2e46d1b5a34d59748376ec34b824bfccefc6b5a",
          "url": "https://github.com/uselessgoddess/dunes/commit/ea41f4eb0c86b9efe0cc74d78cbe487b40df3bc4"
        },
        "date": 1764792579621,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert + Remove (100 elements)",
            "value": 115.94,
            "range": "\u00b1 1.07",
            "unit": "M links/sec",
            "extra": "200 operations in 1725.07 ns/iter"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 81.63,
            "range": "\u00b1 3.18",
            "unit": "M links/sec",
            "extra": "2000 operations in 24499.43 ns/iter"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 61.48,
            "range": "\u00b1 6.12",
            "unit": "M links/sec",
            "extra": "100 operations in 1626.52 ns/iter"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 44.39,
            "range": "\u00b1 0.72",
            "unit": "M links/sec",
            "extra": "1000 operations in 22529.36 ns/iter"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 33.83,
            "range": "\u00b1 0.68",
            "unit": "M links/sec",
            "extra": "10000 operations in 295591.73 ns/iter"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 88.98,
            "range": "\u00b1 0.94",
            "unit": "M links/sec",
            "extra": "200 operations in 2247.69 ns/iter"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 58.38,
            "range": "\u00b1 0.81",
            "unit": "M links/sec",
            "extra": "2000 operations in 34260.18 ns/iter"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 25.77,
            "range": "\u00b1 0.39",
            "unit": "M links/sec",
            "extra": "20000 operations in 776042.38 ns/iter"
          }
        ]
      }
    ]
  }
}
