<header>
<img src="https://raw.githubusercontent.com/uzinfocom-org/website/main/src/images/logo.svg" alt="logo" height="100" align="left">
<h1 style="display: inline">Korrektor</h1>

CXSMXS stack da yozilgan to'liq web platofrma.

[![GitHub top language](https://img.shields.io/github/languages/top/uzinfocom-org/korrektor?style=flat-square&logo=github)](https://github.com/uzinfocom-org/vicardi)
[![Chat](https://img.shields.io/badge/Chat-grey?style=flat-square&logo=telegram)](https://t.me/korrektuz)

[//]: # "[![Test CI](https://github.com/uzinfocom-org/korrektor-rs/actions/workflows/test.yml/badge.svg)](https://github.com/uzinfocom-org/korrektor-rs/actions/workflows/test.yml)"

</header>

## Platforma haqida

Ushbu platformani yozishda UwUssimo tomonidan yaratilgan CXSMXS steki ishlatildi. Dastur 2 qismdan iborat.
Frontend va Backend. Frontend qismi Next.js freymvorkidan foydalanadi. Backend qismida esa da Rust dasturlash
tilida yozilgan Actix freymvorki ishlaydi va bu hamma orkestrani esa yagona monorepo menejeri bo'lmish Turborepo
chalib o'ynatadi. Ushbu loyiha esa hamma korrektordagi qulayliklarni qulay ko'rinishda foydalanuvchilarga yetkazib
berish maqsadini bajaradi.

## Qulayliklar

- Ma'lumotlarni o'zbek alifbosi tartibida saralash
- Tokenizatsiya. O'zbek tili imlo qoidalariga asosan so'zlarni bo'ginlarga ajratish
- Matndagi so'zlar chastotasini hisoblash
- Dublikatlar tozalash
- _Yanada ko'proq imkoniyatlar keyingi relizlarda..._

> Bu loyiha hozir sinov bosqichidan o'tmoqda. Agarda biror xatolikka duchor
> bo'lsangiz, xatolik haqida [xabardor](https://github.com/uzinfocom-org/korrektor/issues/new)
> qilishni unutmang.

## O'rnatish & Ishga tushurish

Ushbu loyihani ishga tushurish uchun sizda quyidagi dasturlar o'rnatilgan bo'lishi lozim:

- [Node.js](https://nodejs.org/en/) & [PNPM](https://pnpm.io)
- [Rust & Cargo](https://www.rust-lang.org/tools/install)

O'rnatib bo'lgach esa loyiha turgan joydan terminal ochib turib, quyidagi buyruqlarni buyruq
satrida ishga tushuramiz:

```shell
pnpm install # hamma kerakli paketlarni o'rnatib olish
pnpm run dev # demonstativ holatda platformani ishga tushurish
```

## Qurish

Loyihani productionga chiqarish uchun ikki qismni qurish va kompilyatsiya qilish lozim. Ushbu protsess
bajarish uchun esa, quyidagi buyruq satrini ishga tushuramiz

```shell
pnpm run build # yoki turbo build
```

## CXSMXS haqida

CXSMXS stek bu UwUssimo Robinson tomonidan yaratilgan web stack hisoblanib, UwUssimoning yillar davomida
to'plangan tajribalarini birlashtirgan holda yaratilgan va ancha produktiv sinovlardan o'tkazish natijasida
tarqatilgan stek hisoblanadi. Ushbu stek eng zamonivy texnologiyalar o'z ichiga olib, produktivlikdan tashqari
dasturchilar ergonomikasi haqida ham qayg'uradi. Stek yaralishidan maqsad esa, dasturchilarga qiyinchilik
tug'dirmagan holda eng tez va produktiv web platforma va yechimlar yaratishdir.

**Ko'proq ma'lumotlar
uchun:** [V1](https://www.uwussi.moe/stack/cxsmxs) | [V2](https://www.uwussi.moe/stack/cxsmxs-v2.0)

## Litsenziya

Ushbu kutubxona AGPL-3.0 litsenziyasi ostida tarqatiladi. Batafsil ma'lumot uchun [LICENSE](./LICENSE) fayllarini ko'zdan kechiring!
