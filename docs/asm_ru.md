# Структура класса

Все классы берут свое начало от класса TObject и их код рантайма жестко завязан на эту структуру.

Все элементы по 4 байта если не сказано иначе.

- указатель на самого себя (selfPtr)
- указатель на interface table или 0
- указатель на auto table или 0
- указатель на init table или 0
- указатель на type info или 0
- указатель на field table или 0
- указатель на method table или 0
- указатель на dynamic table или 0
- указатель на className
- размер инстанса
- указатель на начало родительского класса или 0
- указатель на метод safe call exception # с этого метода и далее (до метода destroy) методы наследуются от предков вплоть до TObject
- метод after construction
- метод before destruction
- метод dispatch
- метод default handler
- метод new instance
- метод free instance
- метод destroy

Перед и после этой структуры могут идти разные таблицы, назначение некоторых из них будет описано ниже.

## Interface table

Состоит из следующих элементов (взято из GPL исходников Delphi 7):
```pascal
PGUID = ^TGUID;
  TGUID = packed record
    D1: LongWord;
    D2: Word;
    D3: Word;
    D4: array[0..7] of Byte;
  end;

  PInterfaceEntry = ^TInterfaceEntry;
  TInterfaceEntry = packed record
    IID: TGUID;
    VTable: Pointer;
    IOffset: Integer;
    ImplGetter: Integer;
  end;

  PInterfaceTable = ^TInterfaceTable;
  TInterfaceTable = packed record
    EntryCount: Integer;
    Entries: array[0..9999] of TInterfaceEntry;
  end;
```

Эта таблица используется при создании объекта класса в методе TObject.InitInstance. Если у класса есть ненулевой указатель на interface table, то при создании объекта в его структуру по смещению IOffset будут записаны указатели VTable (на таблицы интерфейсных(?) методов).

Как происходит вызов этих методов еще нужно выяснить и откуда беруться эти методы нужно еще выяснить.

## Dynamic table

Состоит из следующих элементов: (далее псевдокод на расте)
```rust
struct DynamicTable {
    count: u16, // количество методов
    indexes: Vec<u16>, // индексы-селекторы методов
    methods: Vec<u32>, // указатели на методы
}
```

Методы вызываются с помощью ряда вспомогательных вызовов (`_CallDynaInst`, `_CallDynaClass`) которые в итоге используют метод `GetDynaMethod` для выборки метода. 
Сигнатура: `function GetDynaMethod(vmt: TClass; selector: Smallint): Pointer` (не совсем точная с точки зрения способа передачи аргументов и возврата результата)
Как он работает:
- первый аргумент передается в EAX, второй в SI, возврат в ESI
- если класс имеет указатель на dyntable, то использует его, иначе ищет в цепочке родителей
- если dyntable не найдена, то выход с флагом ZF = 1 (не найдено)
- если указатель найден, то по его адресу берется число методов, потом происходит сравнение переданного селектора (в дополненном коде) с элементами массива indexes
- если найден такой индекс, то выбирается соотв. метод из массива methods, возвращается указатель на метод в ESI и ZF = 0

Скорее всего массив indexes нужен для оптимизации поиска метода, т.к. похоже что элементы массива всегда начинаются с 1 и идут по порядку.
