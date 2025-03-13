import random

def newNum():
    return format(random.randint(0, 255), 'x')

def checkUniqueness(num, table):
    return num not in table

def checkLength(table):
    return len(table) < 256

def AddNewNum(table):
    num = newNum()
    if checkUniqueness(num, table):
        table.append(num)

def fillTable(table):
    while checkLength(table):
        AddNewNum(table)
    return table

def outputTable(table):
    f = open("sBox.txt", 'a')
    for num in table:
        f.write(num + "\n")
    f.close()

def main():
    table = []
    table = fillTable(table)
    outputTable(table)

main()
