use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use rust_decimal::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct DataRecv {
    pub precio: f32,
    pub prestamo: f32,
    pub anos: f32,
    pub interes: f32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DataSend {
    pub balance: Decimal,
    pub mes: u16,
    pub pago_mensual: Decimal,
    pub pago_interes: Decimal,
    pub pago_amor: Decimal,
    pub intereses_totales: Decimal,
    pub pago_total_amor: Decimal,
    pub gasto_total: Decimal,
    pub id: u16,
}
// type Final = DataSend;

#[wasm_bindgen]
pub fn mort_calculator(info: &JsValue) -> JsValue {
    let data: DataRecv = info.into_serde().unwrap();

    let interes = Decimal::from_f32(data.interes).unwrap();

    // let inter = interes / Decimal::from_f32(100.00).unwrap();
    let balance = Decimal::from_f32(data.prestamo).unwrap();

    //esto se usa para calcular la aco
    let mut base = balance.clone();
    let anos = Decimal::from_f32(data.anos).unwrap();

    //decimal de 12
    let DEC12: Decimal = Decimal::from_f32(12.00).unwrap();
    //decimal de 1
    let DEC1: Decimal = Decimal::from_f32(1.00).unwrap();

    let CERO: Decimal = Decimal::from_f32(0.00).unwrap();

    // cuanto paga de interes
    let pagoint = &base * (&interes / &DEC12);

    //amortbase
    let amort_base = &DEC1 + &interes / &DEC12;
    //la potencia para calcular
    let potencia = &DEC12 * &anos;
    use num;

    let amort_pago = num::pow(amort_base, potencia.to_usize().unwrap());

    let pago_mensual = (&pagoint * &amort_pago) / (&amort_pago - &DEC1);

    //conto de meses de transasction
    let mut mes: u16 = 0;
    //id de cada objeto
    let mut id: u16 = 0;
    // total pago interes
    let mut pago_tot_int = Decimal::from_f32(0.00).unwrap();
    // pago total de amortización
    let mut total_pago_amor = Decimal::from_f32(0.00).unwrap();

    let mut total: Vec<DataSend> = Vec::new();

    while base >= CERO {
        if base.round_dp(2) == CERO || total_pago_amor.round_dp(2) == balance.round_dp(2) {
            return JsValue::from_serde(&total).unwrap();
        }
        let pagoint = &base * (&interes / &DEC12);
        pago_tot_int = &pago_tot_int + &pagoint;

        let menos_blc = &pago_mensual - pagoint;

        total_pago_amor = &total_pago_amor + &menos_blc;

        base = &base - menos_blc;

        mes = mes + 1;
        id = id + 1;
        let obj: DataSend = DataSend {
            balance: balance,
            mes: mes.clone(),
            pago_mensual: pago_mensual.round_dp(2).clone(),
            pago_interes: pagoint.round_dp(2).clone(),
            pago_amor: menos_blc.round_dp(2).clone(),
            intereses_totales: pago_tot_int.round_dp(2).clone(),
            pago_total_amor: total_pago_amor.round_dp(2).clone(),
            gasto_total: total_pago_amor.round_dp(2).clone() + pago_tot_int.round_dp(2).clone(),
            id: id.clone(),
        };
        total.push(obj);
    }
    return JsValue::from_serde(&total).unwrap();
}
