# Домашнее задание

Термометр.

Цель: Научиться использовать отдельный поток для I/O задач.

Результатом является: Модуль работы с термометром и его имитатор.

Описание/Пошаговая инструкция выполнения домашнего задания:

* Термометр позволяет узнать температуру, периодически отправляя UDP пакет с данными на заданный адрес.
* Для прослушивания UDP предлагается запустить цикл получения датаграмм в отдельном потоке.
* Также, для проверки нового функционала, реализовать приложение, имитирующее работу термометра.

Критерии оценки:

Статус "Принято" ставится, если:

* Присутствует весь функционал из описания.
* Выполняются тесты функционала из описания.
* Утилита clippy не сообщает об ошибках и не выдаёт предупреждений.
* Утилита fmt не меняет форматирование кода.