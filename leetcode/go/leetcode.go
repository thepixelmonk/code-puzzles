package main

import (
    "os"
    "log"
    "fmt"
    "leetcode/easy"
)

func main() {
    if len(os.Args) < 2 {
        implementations := []string{"TwoSum"}
        fmt.Println("Available implementations:")
        for _, i := range implementations {
            fmt.Println(i)
        }
        return
    }
    switch os.Args[1] {
    case "TwoSum":
        fmt.Println(easy.TwoSum([]int{2,7,11,15}, 9))
    default:
        log.Fatal("No such implementation: ", os.Args[1])
    }
}
