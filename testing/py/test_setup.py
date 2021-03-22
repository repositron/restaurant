import requests
import json

menu_url = "http://localhost:8080/menu"
table_url = "http://localhost:8080/table"

headers = {"content-type": "application/json"}

def create_menu_item(name, cooktime, price):
    jsonStr = f'{{"item_name":"{name}", "cooktimeseconds": {cooktime}, "price": {price}}}'
    print(jsonStr)
    print(json.dumps(jsonStr))
    r = requests.post(menu_url, json=json.loads(jsonStr), headers=headers)
    print(r.status_code)
    print(r.text)

def get_all_menu():
    r = requests.get(menu_url)
    print(r.text)

def create_table_item(name):
    jsonStr = f'{{"code":"{name}"}}'
    r = requests.post(table_url, json=json.loads(jsonStr), headers=headers)
    print(r.status_code)
    print(r.text)

def get_all_table():
    r = requests.get(menu_url)
    print(r.text)

def setup_menu():
    #r = requests.post(test_url, json={"item_name": "pasta", "cooktimeseconds": 20, "price": 500})
    #print(r.text)
    create_menu_item("pasta", 30, 100)
    create_menu_item("pasta_a", 40, 80)
    create_menu_item("curry_a", 50, 100)
    create_menu_item("curry_a", 60, 100)
    create_menu_item("curry_b", 70, 100)
    create_menu_item("curry_c", 80, 100)
    create_menu_item("curry_d", 90, 100)
    create_menu_item("curry_r", 100, 100)
    create_menu_item("curry_e", 130, 100)

def setup_table():
    create_table_item("table1")
    create_table_item("table2")
    create_table_item("table3")
    create_table_item("table4")
    create_table_item("table5")




def main():
    setup_menu()
    get_all_menu()
    setup_table()
    get_all_table()
main()
