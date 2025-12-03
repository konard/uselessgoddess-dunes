window.BENCHMARK_DATA = {
  "lastUpdate": 1764791033098,
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
          "id": "67b680a6ff437a4c9fdd10129db4314ac522724b",
          "message": "Merge pull request #21 from konard/issue-20-7bfd63213319\n\nImprove benchmarks report quality on gh pages",
          "timestamp": "2025-12-03T22:40:34+03:00",
          "tree_id": "cca48970b1e6513b5a077ed3db9adc9401cd83ce",
          "url": "https://github.com/konard/uselessgoddess-dunes/commit/67b680a6ff437a4c9fdd10129db4314ac522724b"
        },
        "date": 1764791032475,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert + Remove (100 elements)",
            "value": 115076784,
            "range": "47884022",
            "unit": "links/sec",
            "extra": "200 operations in 1737.97 ns/iter"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 87228789,
            "range": "7496448",
            "unit": "links/sec",
            "extra": "2000 operations in 22928.21 ns/iter"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 61715823,
            "range": "2726337",
            "unit": "links/sec",
            "extra": "100 operations in 1620.33 ns/iter"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 45358961,
            "range": "820094",
            "unit": "links/sec",
            "extra": "1000 operations in 22046.36 ns/iter"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 33801315,
            "range": "619092",
            "unit": "links/sec",
            "extra": "10000 operations in 295846.47 ns/iter"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 87660089,
            "range": "5980601",
            "unit": "links/sec",
            "extra": "200 operations in 2281.54 ns/iter"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 60044408,
            "range": "631146",
            "unit": "links/sec",
            "extra": "2000 operations in 33308.68 ns/iter"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 26695910,
            "range": "491679",
            "unit": "links/sec",
            "extra": "20000 operations in 749178.43 ns/iter"
          }
        ]
      }
    ]
  }
}