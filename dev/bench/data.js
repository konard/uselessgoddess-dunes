window.BENCHMARK_DATA = {
  "lastUpdate": 1764792808051,
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
          "id": "ea41f4eb0c86b9efe0cc74d78cbe487b40df3bc4",
          "message": "Merge pull request #25 from konard/issue-24-8b8684529f8d\n\nImprove benchmarks quality",
          "timestamp": "2025-12-03T23:08:51+03:00",
          "tree_id": "a2e46d1b5a34d59748376ec34b824bfccefc6b5a",
          "url": "https://github.com/konard/uselessgoddess-dunes/commit/ea41f4eb0c86b9efe0cc74d78cbe487b40df3bc4"
        },
        "date": 1764792807147,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert + Remove (100 elements)",
            "value": 115.54,
            "range": "± 33.43",
            "unit": "M links/sec",
            "extra": "200 operations in 1730.95 ns/iter"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 84.81,
            "range": "± 3.2",
            "unit": "M links/sec",
            "extra": "2000 operations in 23583.32 ns/iter"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 60.53,
            "range": "± 2.1",
            "unit": "M links/sec",
            "extra": "100 operations in 1652.12 ns/iter"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 41.92,
            "range": "± 1.51",
            "unit": "M links/sec",
            "extra": "1000 operations in 23856.04 ns/iter"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 33.2,
            "range": "± 19.41",
            "unit": "M links/sec",
            "extra": "10000 operations in 301225.20 ns/iter"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 86.39,
            "range": "± 1.66",
            "unit": "M links/sec",
            "extra": "200 operations in 2315.12 ns/iter"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 56.74,
            "range": "± 1.65",
            "unit": "M links/sec",
            "extra": "2000 operations in 35248.79 ns/iter"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 26.7,
            "range": "± 0.17",
            "unit": "M links/sec",
            "extra": "20000 operations in 749010.01 ns/iter"
          }
        ]
      }
    ]
  }
}