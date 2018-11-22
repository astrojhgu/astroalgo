#![allow(non_snake_case)]
#![allow(dead_code)]

//use num_traits::float::Float;

#[derive(Clone, Copy, Debug)]
pub struct Cosmology {
    pub omega_m_0: f64,
    pub omega_b_0: f64,
    pub omega_l_0: f64,

    pub h100: f64,
    pub cmb_temp_0: f64,

    pub sigma_8: f64,
    pub primordial_index: f64,

    pub delta_c0: f64,
    pub Y: f64,
}

impl Cosmology {
    pub fn new(
        om0: f64,
        ob0: f64,
        ol0: f64,
        h100: f64,
        ct0: f64,
        s8: f64,
        idx: f64,
        dc0: f64,
        Y: f64,
    ) -> Cosmology {
        Cosmology {
            omega_m_0: om0,
            omega_b_0: ob0,
            omega_l_0: ol0,
            h100: h100,
            cmb_temp_0: ct0,
            sigma_8: s8,
            primordial_index: idx,
            delta_c0: dc0,
            Y: Y,
        }
    }

    pub fn lcdm_cosmo() -> Cosmology {
        Self::new(
            0.3089, 0.0486, 0.6911, 0.6774, 2.7255, 0.8159, 0.9667, 1.686, 0.2454,
        )
    }

    pub fn Omega_m_0(&self) -> f64 {
        self.omega_m_0
    }
    pub fn Omega_b_0(&self) -> f64 {
        self.omega_b_0
    }
    pub fn Omega_l_0(&self) -> f64 {
        self.omega_l_0
    }

    pub fn Omega_cdm_0(&self) -> f64 {
        self.omega_m_0 - self.omega_b_0
    }

    pub fn hubble_0(&self) -> f64 {
        use crate::constants::km_per_mpc;
        self.h100 * 100.0 / km_per_mpc
    }

    pub fn cmb_temp_0(&self) -> f64 {
        self.cmb_temp_0
    }

    pub fn sigma_8(&self) -> f64 {
        self.sigma_8
    }

    pub fn primordial_index(&self) -> f64 {
        self.primordial_index
    }

    pub fn critical_density_now(&self) -> f64 {
        use crate::constants::{pi, G};
        3.0 * self.hubble_0().powi(2) / (8.0 * pi * G)
    }

    pub fn rho_crit_0(&self) -> f64 {
        self.critical_density_now()
    }

    pub fn h100(&self) -> f64 {
        self.h100
    }

    pub fn helium_by_mass(&self) -> f64 {
        self.Y
    }

    pub fn helium_by_number(&self) -> f64 {
        1. / (1. / self.Y - 1.) / 4.
    }

    pub fn y(&self) -> f64 {
        self.helium_by_number()
    }

    pub fn Y(&self) -> f64 {
        self.Y
    }

    pub fn X(&self) -> f64 {
        1.0 - self.Y
    }

    pub fn kg_per_baryon(&self) -> f64 {
        use crate::constants::m_H;
        m_H / (1.0 - self.Y) / (1.0 + self.y())
    }

    pub fn b_per_kg(&self) -> f64 {
        1.0 / self.kg_per_baryon()
    }

    pub fn z_dec(&self) -> f64 {
        150. * (self.omega_b_0 * self.h100.powi(2) / 0.023).powf(0.4) - 1.0
    }

    pub fn a_eq(&self) -> f64 {
        (self.omega_m_0 / self.omega_l_0).powf(1.0 / 3.0)
    }

    pub fn z_eq(&self) -> f64 {
        1.0 / self.a_eq() - 1.0
    }

    pub fn Omh2(&self) -> f64 {
        self.omega_m_0 * self.h100.powi(2)
    }

    pub fn rho_b_z0(&self) -> f64 {
        self.mean_baryon_density(0.0)
    }

    pub fn rho_m_z0(&self) -> f64 {
        self.mean_matter_density(0.0)
    }

    pub fn rho_cdm_z0(&self) -> f64 {
        self.rho_m_z0() - self.rho_b_z0()
    }

    pub fn nH0(&self) -> f64 {
        use crate::constants::m_H;
        (1.0 - self.Y()) * self.rho_b_z0() / m_H
    }

    pub fn nHe0(&self) -> f64 {
        self.y() * self.nH0()
    }

    pub fn ne0(&self) -> f64 {
        self.nH0() + 2.0 * self.nHe0()
    }

    pub fn nH(&self, z: f64) -> f64 {
        self.nH0() * (1.0 + z).powi(3)
    }

    pub fn nHe(&self, z: f64) -> f64 {
        self.nHe0() * (1.0 + z).powi(3)
    }

    pub fn delta_c0(&self) -> f64 {
        self.delta_c0
    }

    pub fn T_cmb_now(&self) -> f64 {
        self.cmb_temp_0
    }

    pub fn fbaryon(&self) -> f64 {
        self.omega_b_0 / self.omega_m_0
    }

    pub fn fcdm(&self) -> f64 {
        self.Omega_cdm_0() / self.omega_m_0
    }

    pub fn fbar_over_fcdm(&self) -> f64 {
        self.fbaryon() / self.fcdm()
    }

    pub fn evolution_function(&self, z: f64) -> f64 {
        self.omega_m_0 * (1.0 + z).powi(3) + self.omega_l_0
    }

    pub fn hubble_parameter(&self, z: f64) -> f64 {
        self.hubble_0() * self.evolution_function(z).sqrt()
    }

    pub fn hubble_length(&self, z: f64) -> f64 {
        use crate::constants::c;
        c / self.hubble_parameter(z)
    }

    pub fn hubble_time(&self, z: f64) -> f64 {
        1.0 / self.hubble_parameter(z)
    }

    pub fn time2z(&self, ti: f64, tf: f64, zi: f64) -> f64 {
        ((1.0 + zi).powf(-1.5) + (3. * self.hubble_0() * self.omega_m_0.sqrt() * (tf - ti) / 2.))
            .powf(-2. / 3.)
            - 1.0
    }

    pub fn lookback_time(&self, zi: f64, zf: f64) -> f64 {
        self.t_of_z(zi) - self.t_of_z(zf)
    }

    pub fn T_cmb(&self, z: f64) -> f64 {
        self.cmb_temp_0 * (1.0 + z)
    }

    pub fn U_cmb(&self, z: f64) -> f64 {
        use crate::constants::{c, sigma_SB};
        4.0 * sigma_SB() * self.T_cmb(z).powi(4) / c
    }

    pub fn t_of_z(&self, z: f64) -> f64 {
        let a = 1.0 / (1.0 + z);
        let t = (2. / 3. / (1.0 - self.omega_m_0).sqrt())
            * ((a / self.a_eq()).powf(1.5) + (1. + (a / self.a_eq()).powi(3)).sqrt()).ln()
            / self.hubble_0();
        t
    }

    pub fn T_gas(&self, z: f64) -> f64 {
        let zd = self.z_dec();
        if z >= zd {
            self.T_cmb(z)
        } else {
            self.T_cmb(zd) * (1.0 + z).powi(2) / (1.0 + zd).powi(2)
        }
    }

    pub fn Omega_matter(&self, z: f64) -> f64 {
        self.omega_m_0 * (1.0 + z).powi(3) / self.evolution_function(z)
    }

    pub fn Omega_Lambda(&self, z: f64) -> f64 {
        self.omega_l_0 / self.evolution_function(z)
    }

    pub fn mean_matter_density(&self, z: f64) -> f64 {
        self.Omega_matter(z) * self.critical_density(z)
    }

    pub fn mean_baryon_density(&self, z: f64) -> f64 {
        self.omega_b_0 / self.omega_m_0 * self.mean_matter_density(z)
    }

    pub fn mean_hydrogen_number_density(&self, z: f64) -> f64 {
        use crate::constants::m_H;
        (1.0 - self.Y()) * self.mean_baryon_density(z) / m_H
    }

    pub fn mean_helium_number_density(&self, z: f64) -> f64 {
        use crate::constants::m_He;
        self.Y() * self.mean_baryon_density(z) / m_He
    }

    pub fn critical_density(&self, z: f64) -> f64 {
        use crate::constants::{pi, G};
        3.0 * self.hubble_parameter(z).powi(2) / (8.0 * pi * G)
    }

    pub fn dtdz(&self, z: f64) -> f64 {
        1.0 / self.hubble_parameter(z) / (1.0 + z)
    }

    pub fn luminosity_distance(&self, z: f64) -> f64 {
        use crate::constants::c;
        use scorus::integration::simpsons_int;
        let integr = simpsons_int(
            &|z: f64| self.hubble_0() / self.hubble_parameter(z),
            0.0,
            z,
            1e-6,
            20,
        ).unwrap();
        integr * c * (1.0 + z) / self.hubble_0()
    }

    pub fn comoving_radial_distance(&self, z0: f64, z: f64) -> f64 {
        use crate::constants::c;
        use scorus::integration::simpsons_int;
        let integr = simpsons_int(
            &|z: f64| self.hubble_0() / self.hubble_parameter(z),
            z0,
            z,
            1e-6,
            20,
        ).unwrap();
        return c * integr / self.hubble_0();
    }

    pub fn proper_radial_distance(&self, z0: f64, z: f64) -> f64 {
        self.comoving_radial_distance(z0, z) / (1.0 + z0)
    }

    pub fn comoving_line_element(&self, z: f64) -> f64 {
        use crate::constants::c;
        c / self.hubble_parameter(z)
    }

    pub fn proper_line_element(&self, z: f64) -> f64 {
        self.comoving_line_element(z) / (1.0 + z)
    }

    pub fn dldz(&self, z: f64) -> f64 {
        self.proper_line_element(z)
    }

    pub fn critical_density_for_collapse(&self, z: f64) -> f64 {
        use crate::constants::pi;
        let d = self.Omega_matter(z) - 1.0;
        18.0 * pi.powi(2) + 82.0 * d - 39.0 * d.powi(2)
    }
}
