use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use rust_decimal::prelude::*;

use rust_decimal_macros::*;

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

    let inter = interes / Decimal::from_f32(100.00).unwrap();
    let mut base = Decimal::from_f32(data.prestamo).unwrap();
    let anos = Decimal::from_f32(data.anos).unwrap();

    //decimal de 12
    let DEC12: Decimal = Decimal::from_f32(12.00).unwrap();
    //decimal de 1
    let DEC1: Decimal = Decimal::from_f32(1.00).unwrap();

    // cuanto paga de interes
    let pagoint = base * (&inter / DEC12);

    //amortbase
    let amort_base = DEC1 + &inter / DEC12;
    //la potencia para calcular
    let potencia = DEC12 * &anos;
    use num;

    let amort_pago = num::pow(amort_base, potencia.to_usize().unwrap());

    let pago_mensual = (&pagoint * &amort_pago) / (&amort_pago - DEC1);

    //conto de meses de transasction
    let mut mes: u16 = 0;
    //id de cada objeto
    let mut id: u16 = 0;
    // total pago interes
    let mut pago_tot_int = Decimal::from_f32(0.00).unwrap();
    // pago total de amortización
    let mut total_pago_amor = Decimal::from_f32(0.00).unwrap();

    let mut total: Vec<DataSend> = Vec::new();

    while base >= Decimal::from_f32(0.00).unwrap() {
        let pagoint = &base * (&inter / DEC12);
        pago_tot_int = &pago_tot_int + &pagoint;

        let menos_blc = &pago_mensual - pagoint;

        total_pago_amor = &total_pago_amor + &menos_blc;

        base = &base - menos_blc;

        mes = mes + 1;
        id = id + 1;
        let obj: DataSend = DataSend {
            balance: base.clone(),
            mes: mes.clone(),
            pago_mensual: pago_mensual.clone(),
            pago_interes: pagoint.clone(),
            pago_amor: menos_blc.clone(),
            intereses_totales: pago_tot_int.clone(),
            pago_total_amor: total_pago_amor.clone(),
            gasto_total: total_pago_amor.clone() + pago_tot_int.clone(),
            id: id.clone(),
        };
        total.push(obj);
    }
    return JsValue::from_serde(&total).unwrap();
}
