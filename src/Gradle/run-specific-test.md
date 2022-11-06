# Run tests using Gradle

## Run a single test:

```
./gradlew testDebugUnitTest --tests TestClassNameHere.testFunctionHere
```

### Rerun tests when up-to-date

`--rerun-tasks`

or 

`test.outputs.upToDateWhen {false}` in the config
