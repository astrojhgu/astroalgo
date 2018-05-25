#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]


pub const pi: f64 = 3.14159265358979323846;
pub const cm_per_m: f64 = 1e2;
pub const m_per_cm: f64 = 1e-2;
pub const km_per_pc: f64 = 3.08568e13;
pub const km_per_mpc: f64 = km_per_pc * 1e6;
pub const km_per_gpc: f64 = km_per_mpc * 1e3;
pub const m_per_pc: f64 = km_per_pc * 1e3;
pub const m_per_kpc: f64 = m_per_pc * 1e3;
pub const m_per_mpc: f64 = m_per_pc * 1e6;
pub const m_per_gpc: f64 = m_per_mpc * 1e3;
pub const m_per_km: f64 = 1e3;
pub const m_per_rsun: f64 = 695500. * m_per_km;
pub const m_per_rEarth: f64 = 6371000.;
pub const m_per_au: f64 = 1.49597871e11;
pub const barn: f64 = 1e-28;
pub const Mbarn: f64 = barn * 1e6;
// masses
pub const kg_per_amu: f64 = 1.660538921e-27;
pub const kg_per_msun: f64 = 1.98892e30;
pub const mH_amu: f64 = 1.00794;
pub const mHe_amu: f64 = 4.002602;

// times
pub const s_per_yr: f64 = 365.25 * 24.0 * 3600.0;
pub const s_per_kyr: f64 = s_per_yr * 1e3;
pub const s_per_myr: f64 = s_per_kyr * 1e3;
pub const s_per_gyr: f64 = s_per_myr * 1e3;

// angles
pub const sqdeg_per_std: f64 = 180.0 * 180. / (pi * pi);

// General constants
pub const h_P: f64 = 6.626068e-34; // Planck's constant - [h] = J*s
pub const h_p: f64 = h_P;
pub const h: f64 = h_p;
pub const h_bar: f64 = h / 2. / pi; // H-bar - [h_bar] = J*s
pub const c: f64 = 299792458.0; // Speed of light - [c] = m/s
pub const k_B: f64 = 1.3806503e-23; // Boltzmann's constant - [k_B] = J/K
pub const G: f64 = 6.673e-11; // Gravitational constant - [G] = m^3/kg/s^2
pub const e: f64 = 1.60217646e-19; // Electron charge - [e] = C
pub const e_cgs: f64 = 4.803204e-10; // Electron charge - [e] = statC
pub const m_e: f64 = 9.10938188e-31; // Electron mass - [m_e] = kg
pub const m_p: f64 = 1.67262158e-27; // Proton mass - [m_p] = kg
pub const m_n: f64 = 1.67492729e-27; // Neutron mass - [m_n] = kg
pub const sigma_T: f64 = 6.65e-29; // Cross section for Thomson scattering - [sigma_T] = m^2
pub const alpha_FS: f64 = 1. / 137.035999070; // Fine structure constant - unitless
pub const Ryd: f64 = 2.1798719e-18; // Rydberg in J

// energies / wavelengths / frequencies
pub const erg_per_j: f64 = 1e7;
pub const j_per_erg: f64 = 1e-7;
pub const j_per_ev: f64 = e;
//pub const erg_per_ev: f64 = e * erg_per_j;
//pub const erg_per_kev: f64 = 1e3 * erg_per_ev;
// Convert specific intensities from eV^-1 to Hz^-1
pub const ev_per_hz: f64 = h / j_per_ev;

// Stefan-Boltzmann constant - [sigma_SB] = J / m^2 / deg^4 / s
pub fn sigma_SB() -> f64 {
    2.0 * pi.powi(5) * k_B.powi(4) / 15.0 / (c * c) / h.powi(3)
}

// Hydrogen
pub const A10: f64 = 2.85e-15; // HI 21cm spontaneous emission coefficient - [A10] = Hz
pub const E10: f64 = 5.9e-6; // Energy difference between hyperfine states - [E10] = eV
pub const m_H: f64 = m_p + m_e; // Mass of a hydrogen atom - [m_H] = kg
pub const nu_0: f64 = 1420.4057e6; // Rest frequency of HI 21cm line - [nu_0] = Hz
                                   // pub const nu_alpha = 2.47e15;         // Rest frequency of Lyman-alpha - [nu_alpha]
                                   // = Hz
pub const T_star: f64 = 0.068; // Corresponding temperature difference between HI hyperfine states - [T_star] = K
pub const a_0: f64 = 5.292e-11; // Bohr radius - [a_0] = m
pub const f12: f64 = 0.4162; // Lyman-alpha oscillator strength

pub const E_LL: f64 = Ryd / j_per_ev; //eV
pub const E_LyA: f64 = E_LL * (1. - 1. / 4.); //eV
pub const E_LyB: f64 = E_LL * (1. - 1. / 9.); //eV
pub const nu_alpha: f64 = E_LyA * j_per_ev / h; //Hz

pub const nu_beta: f64 = E_LyB * j_per_ev / h;
pub const nu_LL: f64 = E_LL * j_per_ev / h;
pub const dnu: f64 = nu_LL - nu_alpha;
pub const nu_0_mhz: f64 = nu_0 / 1e6;

// Helium
pub const m_HeI: f64 = 2.0 * (m_p + m_n + m_e);
pub const m_He: f64 = m_HeI;
pub const m_HeII: f64 = 2.0 * (m_p + m_n) + m_e;
pub const Y: f64 = 0.2477; // Primordial helium abundance by mass
pub const y: f64 = Y / 4. / (1. - Y); // Primordial helium abundance by number
