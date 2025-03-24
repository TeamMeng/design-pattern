trait StoreSDK {
    fn store(&self);
}

struct AliyunSDK;
impl StoreSDK for AliyunSDK {
    fn store(&self) {
        println!("Ali yun store");
    }
}

struct TencentSDK;
impl StoreSDK for TencentSDK {
    fn store(&self) {
        println!("Tencent store");
    }
}

struct QiniuSDK;
impl StoreSDK for QiniuSDK {
    fn store(&self) {
        println!("Qiniu store");
    }
}

struct Account;
impl Account {
    fn store_by_sdk(&self, sdk: &dyn StoreSDK) {
        sdk.store();
    }
}

fn main() {
    let account = Account;

    let a = AliyunSDK;
    let t = TencentSDK;
    let q = QiniuSDK;

    account.store_by_sdk(&a);
    account.store_by_sdk(&t);
    account.store_by_sdk(&q);
}
