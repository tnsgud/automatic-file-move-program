# Automatic File Move Program

선택한 폴더에 있는 파일의 생성된 시간을 확인하여 설정한 기간이 지났을 경우 다른 서버로 이동시키는 프로그램입니다.

`config.example.json` 파일을 참고하여 `config.json` 파일을 생성하고 설정값을 입력해주세요.

`from_target_path`: 파일을 확인할 폴더의 경로입니다.
`to_target_path`: 파일을 이동시킬 폴더의 경로입니다.
`target_diff`: 파일을 이동시킬 시간입니다. (단위: day, number)
