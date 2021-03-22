import requests
import pytest
import json

menu_url = "http://localhost:8080/menu"
table_url = "http://localhost:8080/table"
session_url = "http://localhost:8080/session"

headers = {"content-type": "application/json"}

def get_all_session():
    r = requests.get(session_url)
    print(r.text)


def create_session(customer_table_id):
    jsonStr = f'{{"customer_table_id":{customer_table_id}}}'
    r = requests.post(session_url, json=json.loads(jsonStr), headers=headers)
    print(r.status_code)
    print(r.text)
    return r

def test_order_200():
    r = create_session(1)
    assert r.status_code == 200
    get_all_session()



#@pytest.fixture
