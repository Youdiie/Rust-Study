// crypto-js 모듈 불러오기
import * as CryptoJS from 'crypto-js';

// 암호화 할 데이터
import * as data from './untitled2.json';

// 복호화 키 지정
const privateKey = 'secret key';

// 시간 측정
console.log(new Date());

// AES알고리즘 사용 암호화
const encrypted = CryptoJS.AES.encrypt(JSON.stringify(data), privateKey).toString();
console.log(new Date());

// AES알고리즘 사용 복호화 ( 복구 키 필요 )
const bytes = CryptoJS.AES.decrypt(encrypted, privateKey);

// 인코딩, 문자열로 변환, JSON 변환
const decrypted = JSON.parse(bytes.toString(CryptoJS.enc.Utf8));
console.log(new Date());

// console.log(encrypted);
// console.log(decrypted);