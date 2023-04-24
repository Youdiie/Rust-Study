import os
import hashlib
import base64
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives import padding
import time

# 비밀 키 생성
key = "oingisprettyintheworld1234567890"  # 임의의 문자열
keyBytes = hashlib.sha256(key.encode()).digest()

# 초기화 벡터 생성
# ivBytes = os.urandom(16)
ivBytes = b"\xda\xe4\x87\xa7i\x8arbOC\xbd \x02\xbc*["
print(type(ivBytes))

start = time.time()
file_name = "1024.jpg"
# file_name = "2048.jpg"

# 이미지 파일을 바이트 배열로 읽기
with open(file_name, "rb") as f:
    imageBytes = f.read()

# AES 256/CBC/PKCS7Padding 설정
cipher = Cipher(algorithms.AES(keyBytes), modes.CBC(ivBytes), backend=default_backend())

# 이미지 바이트 배열을 암호화
encryptor = cipher.encryptor()
padder = padding.PKCS7(algorithms.AES.block_size).padder()
imageBytes = padder.update(imageBytes) + padder.finalize()
encryptedImageBytes = encryptor.update(imageBytes) + encryptor.finalize()

# Base64 인코딩 적용
# encodedImageString = base64.b64encode(encryptedImageBytes).decode()

# 결과 출력
encrpyted_file_name = "encrypted_image" + file_name
with open(encrpyted_file_name, "wb") as f:
    f.write(encryptedImageBytes)
end_encrypt = time.time()
print(f"Encrypt {file_name} Finished at: {end_encrypt-start}")

#############
## Decrypt ##
#############

# Base64 디코딩 적용
# encryptedImageBytes = base64.b64decode(encodedImageString)

# AES 256/CBC/PKCS7Padding 설정
cipher = Cipher(algorithms.AES(keyBytes), modes.CBC(ivBytes), backend=default_backend())

# 이미지 바이트 배열을 복호화
decryptor = cipher.decryptor()
unpadder = padding.PKCS7(algorithms.AES.block_size).unpadder()
decryptedImageBytes = decryptor.update(encryptedImageBytes) + decryptor.finalize()
imageBytes = unpadder.update(decryptedImageBytes) + unpadder.finalize()
print(imageBytes)

# 이미지 파일로 저장하기
decrpyted_file_name = "decrypted_image" + file_name
with open(decrpyted_file_name, "wb") as f:
    f.write(imageBytes)

end_decrypt = time.time()
print(f"Decrypt {file_name} Finished at: {end_decrypt-end_encrypt}")
