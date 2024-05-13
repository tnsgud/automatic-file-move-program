import json
import os
import shutil
from datetime import datetime

from_target_path = ''
to_target_path = ''
target_diff = 0
target_list = []

def load_config():
  global from_target_path, to_target_path, target_diff

  with open('./config.json') as json_file:
    obj = json.load(json_file)

    from_target_path = obj['from_target_path']
    to_target_path = obj['to_target_path']
    target_diff = obj['target_diff']

def find_file():
  global from_target_path, target_diff

  targets = []
  children = os.listdir(from_target_path)

  for child in children:
    if child.startswith('.'): continue

    stat = os.stat(f'{from_target_path}/{child}')
    creation_time = datetime.fromtimestamp(stat.st_ctime)
    diff = datetime.now() - creation_time

    if diff.days > target_diff:
      targets.append(child)

  return targets

def move_file():
  global to_target_path, target_list 

  for child in target_list:
    shutil.move(f'{from_target_path}/{child}', f'{to_target_path}/{child}')


load_config()

target_list = find_file()
move_file()

